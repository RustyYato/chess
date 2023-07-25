use std::{
    collections::{hash_map::Entry, HashMap},
    error::Error,
    io::{BufReader, Read},
};

use bstr::ByteSlice;
use chess_bitboard::{File, Pos, Rank};
use chess_movegen::{Board, ChessMove};
use pgn_reader::BufferedReader;

#[derive(Debug, serde::Serialize)]
struct MoveTrie {
    count: u32,
    depth: usize,
    #[serde(serialize_with = "serialize_chess_move")]
    next: HashMap<ChessMove, MoveTrie>,
}

#[derive(Debug, serde::Deserialize)]
struct DeMoveTrie {
    count: u32,
    depth: usize,
    next: HashMap<DeChessMove, DeMoveTrie>,
}

impl From<DeMoveTrie> for MoveTrie {
    fn from(value: DeMoveTrie) -> Self {
        MoveTrie {
            count: value.count,
            depth: value.depth,
            next: value
                .next
                .into_iter()
                .map(|(mv, next)| (mv.mv, next.into()))
                .collect(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct DeChessMove {
    mv: ChessMove,
}

impl<'de> serde::de::Deserialize<'de> for DeChessMove {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let x: &str = serde::de::Deserialize::deserialize(deserializer)?;
        Ok(Self {
            mv: ChessMove::from_ascii_bytes(x.as_bytes()).unwrap(),
        })
    }
}

fn serialize_chess_move<S: serde::Serializer>(
    next: &HashMap<ChessMove, MoveTrie>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    use serde::ser::SerializeMap;

    let mut map = serializer.serialize_map(Some(next.len()))?;

    for (mv, next) in next {
        map.serialize_entry(&mv.to_string(), next)?;
    }

    map.end()
}

#[allow(clippy::type_complexity)]
pub fn read_lichess_games() -> Result<Vec<u16>, Box<dyn Error>> {
    let s = std::fs::read("temp/moves_trie.json")?;
    let start = std::time::Instant::now();
    let x: DeMoveTrie = serde_json::from_slice(&s)?;
    dbg!(start.elapsed());
    drop(s);
    dbg!(start.elapsed());
    let mut trie = MoveTrie::from(x);
    dbg!(start.elapsed());

    dbg!(trie.depth);
    trie.validate(0);
    trie.trim(0);

    let writer = std::fs::File::create("temp/moves_trie_trim.json")?;
    let writer = std::io::BufWriter::new(writer);
    serde_json::to_writer_pretty(writer, &trie)?;

    let mut data = Vec::new();
    encode(&mut trie, &mut data, 0);

    return Ok(data);

    todo!();
    let reader = BufReader::new(std::fs::File::open(
        "temp/lichess_db_standard_rated_2023-06.pgn.zst",
    )?);
    let mut reader = zstd::Decoder::new(reader)?;
    let mut buffer = Vec::new();

    let mut trie = std::thread::scope(|s| {
        let mut channels_list = Vec::new();
        let mut joins = Vec::new();

        for _ in 0..8 {
            let (tx, rx) = std::sync::mpsc::sync_channel(100);

            let j = s.spawn(move || {
                let mut trie = MoveTrie {
                    count: 0,
                    depth: 0,
                    next: HashMap::new(),
                };

                loop {
                    let x: Vec<u8> = match rx.recv() {
                        Ok(x) => x,
                        Err(_) => break Ok::<_, std::io::Error>(trie),
                    };

                    let mut reader = BufferedReader::new(x.as_slice());
                    let mut visitor = Visitor {
                        both_high_elo: 0,
                        games: 0,
                        nodes: 0,
                        committed_nodes: 0,
                        board: Board::standard(),
                        moves: Vec::new(),
                        trie: &mut trie,
                        max_counts: Vec::new(),
                    };
                    while reader.read_game(&mut visitor)?.is_some() {}
                }
            });

            joins.push(j);

            channels_list.push(tx);
        }

        let mut channels = channels_list.iter_mut();

        let start = std::time::Instant::now();
        let mut games = 0;
        let mut bytes_read = 0;
        loop {
            let x = reader.by_ref().take(4 * 4096).read_to_end(&mut buffer)?;
            bytes_read += x;

            if x == 0 {
                break;
            }

            while buffer.starts_with(b"[Event") {
                let next_event = buffer[1..].find(b"[Event");
                if let Some(next_event) = next_event {
                    games += 1;
                    // eprintln!("{:=>100}", "");
                    // eprintln!("{}", bstr::BStr::new(&buffer[..next_event + 1]));
                    let x = buffer.splice(..next_event + 1, std::iter::empty());

                    let channel = match channels.next() {
                        Some(channel) => channel,
                        None => {
                            channels = channels_list.iter_mut();
                            channels.next().unwrap()
                        }
                    };

                    channel.send(Vec::from_iter(x)).unwrap();

                    eprint!(
                        "\r{} games decoded at {}/s",
                        readable::Int::from(games),
                        bytesize::ByteSize::b(
                            (bytes_read as u128 / (start.elapsed().as_millis() + 1) * 1000) as u64
                        )
                    );
                } else {
                    break;
                }
            }
        }

        eprintln!();

        drop(channels_list);

        let mut trie = MoveTrie {
            count: 0,
            depth: 0,
            next: HashMap::new(),
        };
        for j in joins {
            trie.merge(j.join().unwrap()?);
        }

        dbg!(start.elapsed());

        Ok::<_, Box<dyn Error>>(trie)
    })?;

    let mut data = Vec::new();

    let writer = std::fs::File::create("temp/moves_trie.json")?;
    let writer = std::io::BufWriter::new(writer);
    serde_json::to_writer_pretty(writer, &trie)?;

    encode(&mut trie, &mut data, 0);
    dbg!(data.len());

    trie.validate(0);

    Ok(data)
}

impl MoveTrie {
    pub fn merge(&mut self, other: Self) -> usize {
        self.count += other.count;

        for (mv, trie) in other.next {
            let depth = match self.next.entry(mv) {
                Entry::Occupied(entry) => entry.into_mut().merge(trie),
                Entry::Vacant(entry) => {
                    let depth = trie.depth;
                    entry.insert(trie);
                    depth
                }
            };

            self.depth = self.depth.max(depth + 1);
        }

        self.depth
    }

    pub fn trim(&mut self, depth: u32) -> bool {
        let mut to_remove = Vec::new();

        for (&mv, next) in self.next.iter_mut() {
            if next.trim(depth + 1) {
                println!("trim");
                to_remove.push(mv);
                self.count -= next.count;
            } else {
                println!("no trim");
            }
        }

        if self.next.len() == to_remove.len() && !self.next.is_empty() {
            println!("remove all")
        }

        for mv in to_remove {
            self.next.remove(&mv);
        }

        if self.count <= 400 {
            return true;
        }

        if self.next.is_empty() {
            depth != 8
        } else {
            false
        }
    }

    pub fn validate(&self, depth: u32) {
        let mut count = 0;
        for (_, next) in self.next.iter() {
            next.validate(depth + 1);
            count += next.count;
        }

        if self.next.is_empty() {
            assert_eq!(depth, 8);
        } else {
            assert!(self.count == count, "count = {count}\n{self:#?}")
        }
    }
}

const COMMIT_THRESHOLD: u32 = 100;

fn encode(trie: &mut MoveTrie, data: &mut Vec<u16>, depth: usize) {
    if trie.count < COMMIT_THRESHOLD {
        return;
    }

    if trie.depth + depth < 5 {
        dbg!();
        return;
    }

    let mut next = Vec::from_iter(trie.next.drain());
    next.sort_unstable_by_key(|(_mv, x)| x.count);

    for (mv, ref mut next) in next {
        let src = mv.source as u16;
        let dest = mv.dest as u16;
        let encoded_mv = src | (dest << 6) | 1 << 15;

        let start = data.len();
        data.push(0);
        encode(next, data, depth + 1);
        data.push(encoded_mv);
        let end = data.len();

        let len: u16 = (end - start).try_into().unwrap();
        assert!((len as usize) <= data.len(), "{len} >= {}", data.len());
        data.push(len);
    }
}

pub struct Visitor<'a> {
    board: Board,
    both_high_elo: u8,
    moves: Vec<ChessMove>,
    trie: &'a mut MoveTrie,
    games: usize,
    nodes: u32,
    committed_nodes: u32,
    max_counts: Vec<u32>,
}

impl pgn_reader::Visitor for Visitor<'_> {
    type Result = ();

    fn end_game(&mut self) -> Self::Result {
        if self.moves.len() != 8 {
            return;
        }

        let mut node = &mut *self.trie;
        let max_depth = self.moves.len();
        node.depth = node.depth.max(max_depth);

        node.count += 1;
        for (depth, mv) in self.moves.drain(..).enumerate() {
            let max_depth = max_depth - depth - 1;
            assert!(mv.piece.is_none());
            node = node.next.entry(mv).or_insert_with(|| {
                self.nodes += 1;
                MoveTrie {
                    count: 0,
                    depth: 0,
                    next: HashMap::new(),
                }
            });
            node.count += 1;
            node.depth = node.depth.max(max_depth);
            self.committed_nodes += u32::from(node.count == COMMIT_THRESHOLD);
            if self.max_counts.len() == depth {
                self.max_counts.push(node.count)
            } else {
                let count = &mut self.max_counts[depth];
                *count = node.count.max(*count);
            }
        }
    }

    fn begin_game(&mut self) {
        self.board = Board::standard();
        self.moves.clear();
        self.games += 1;
    }

    fn begin_headers(&mut self) {
        self.both_high_elo = 0;
    }

    fn header(&mut self, key: &[u8], value: pgn_reader::RawHeader<'_>) {
        if key == b"WhiteElo" || key == b"BlackElo" {
            let elo: u32 = value.decode_utf8().unwrap().parse().unwrap();

            self.both_high_elo += u8::from(elo < 1800);
        }
    }

    fn end_headers(&mut self) -> pgn_reader::Skip {
        pgn_reader::Skip(self.both_high_elo != 2)
    }

    fn san(&mut self, san_plus: pgn_reader::SanPlus) {
        if self.moves.len() >= 8 || self.both_high_elo != 2 {
            return;
        }

        let san = san_plus.san;

        let mv = match san {
            pgn_reader::San::Normal {
                role,
                file,
                rank,
                capture: _,
                to,
                promotion,
            } => {
                let promotion = match promotion {
                    Some(_) => {
                        self.both_high_elo = 0;
                        return;
                    }
                    None => None,
                };

                let dest =
                    chess_bitboard::Pos::new(convert_file(to.file()), convert_rank(to.rank()));
                let file = file.map(convert_file);
                let rank = rank.map(convert_rank);

                let mut rule_c = 0;
                let piece = role
                    .upper_char()
                    .encode_utf8(core::slice::from_mut(&mut rule_c))
                    .parse::<chess_bitboard::Piece>()
                    .unwrap();

                let mut moves = self.board.legals();
                // moves.set_mask(dest.into());

                let mut only_legal = None;
                for mv in &mut moves {
                    if self.board.raw().piece_of(mv.source) != Some(piece) {
                        continue;
                    }

                    if mv.dest != dest {
                        continue;
                    }

                    if let Some(file) = file {
                        if file != mv.source.file() {
                            continue;
                        }
                    }

                    if let Some(rank) = rank {
                        if rank != mv.source.rank() {
                            continue;
                        }
                    }

                    if promotion != mv.piece {
                        continue;
                    }

                    assert!(only_legal.is_none());

                    only_legal = Some(mv);
                }

                only_legal.unwrap()
            }
            pgn_reader::San::Castle(castle) => {
                pub const BACKRANK: [Rank; 2] = [Rank::_1, Rank::_8];

                let king_to_file = convert_file(castle.king_to_file());
                let backrank = BACKRANK[self.board.turn()];
                let king_file = File::E;

                ChessMove {
                    source: Pos::new(king_file, backrank),
                    dest: Pos::new(king_to_file, backrank),
                    piece: None,
                }
            }
            pgn_reader::San::Put { role: _, to: _ } | pgn_reader::San::Null => unreachable!(),
        };

        assert!(self.board.move_mut(mv));
        self.moves.push(mv);
    }
}

fn convert_file(file: pgn_reader::File) -> chess_bitboard::File {
    chess_bitboard::File::all().nth(file as usize).unwrap()
}
fn convert_rank(rank: pgn_reader::Rank) -> chess_bitboard::Rank {
    chess_bitboard::Rank::all().nth(rank as usize).unwrap()
}

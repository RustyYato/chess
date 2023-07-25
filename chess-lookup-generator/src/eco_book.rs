use std::{collections::HashMap, error::Error};

use chess_bitboard::{File, Pos, Rank};
use chess_movegen::{Board, ChessMove};

const VOLA: &str = include_str!("eco/vola.txt");
const VOLB: &str = include_str!("eco/volb.txt");
const VOLC: &str = include_str!("eco/volc.txt");
const VOLD: &str = include_str!("eco/vold.txt");
const VOLE: &str = include_str!("eco/vole.txt");

#[derive(Debug)]
struct MoveTrieMap {
    name: Option<&'static str>,
    next: HashMap<ChessMove, MoveTrieMap>,
}

#[allow(clippy::type_complexity)]

pub fn read_eco() -> Result<(Vec<u16>, Vec<&'static str>), Box<dyn Error>> {
    let volumes = [VOLA, VOLB, VOLC, VOLD, VOLE];

    let mut trie = MoveTrieMap {
        name: None,
        next: HashMap::new(),
    };

    let mut name_indices = HashMap::new();
    let mut names = Vec::new();

    for opening in volumes.into_iter().flat_map(str::lines) {
        let opening = opening.trim_start();

        if opening.is_empty() || opening.starts_with("ignore") {
            continue;
        }

        let opening = opening.strip_prefix(['A', 'B', 'C', 'D', 'E']).unwrap();
        let opening = opening
            .strip_prefix(['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'])
            .unwrap();
        let opening = opening
            .strip_prefix(['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'])
            .unwrap();
        let opening = opening.strip_prefix(':').unwrap_or(opening).trim_start();

        let colon = opening.find(':').unwrap();
        let (name, opening) = opening.split_at(colon);
        let opening = &opening[1..];

        let mut reader = pgn_reader::BufferedReader::new(opening.as_bytes());

        let moves = reader
            .read_game(&mut Visitor {
                board: Board::standard(),
                moves: Vec::new(),
            })?
            .unwrap();

        let mut node = &mut trie;
        for mv in moves {
            assert!(mv.piece.is_none());
            node = node.next.entry(mv).or_insert_with(|| MoveTrieMap {
                name: None,
                next: HashMap::new(),
            });
        }

        assert!(node.name.is_none(), "{}", node.name.unwrap());
        node.name = Some(name);

        let id = name_indices.len() + 1;
        name_indices.entry(name).or_insert_with(|| {
            names.push(name);
            id
        });
    }

    let mut encoded_trie = Vec::new();

    encoded_trie.push(0);
    assert!(name_indices.len() < u16::MAX as usize);
    encode(&trie, &mut encoded_trie, 0);

    Ok((encoded_trie, names))
}

fn encode(trie: &MoveTrieMap, data: &mut Vec<u16>, depth: u32) {
    if let Some(name) = trie.name {
        for _ in 0..depth {
            print!("  ");
        }
        println!("{}", name);
    }

    for (mv, next) in &trie.next {
        for _ in 0..depth {
            print!("  ");
        }
        let src = mv.source as u16;
        let dest = mv.dest as u16;
        let encoded_mv = src | (dest << 6) | 1 << 15;
        println!("{mv} = {encoded_mv}");

        let start = data.len();
        data.push(0);
        encode(next, data, depth + 1);
        data.push(encoded_mv);
        let end = data.len();

        let len: u16 = (end - start).try_into().unwrap();
        assert!((len as usize) <= data.len(), "{len} >= {}", data.len());
        data.push(len);

        for _ in 0..depth {
            print!("  ");
        }
        println!("len = {len}");
    }
}

pub struct Visitor {
    board: Board,
    moves: Vec<ChessMove>,
}

impl pgn_reader::Visitor for Visitor {
    type Result = Vec<ChessMove>;

    fn end_game(&mut self) -> Self::Result {
        core::mem::take(&mut self.moves)
    }

    fn san(&mut self, san_plus: pgn_reader::SanPlus) {
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
                    Some(pgn_reader::Role::Knight) => todo!(),
                    Some(pgn_reader::Role::Bishop) => todo!(),
                    Some(pgn_reader::Role::Rook) => todo!(),
                    Some(pgn_reader::Role::Queen) => todo!(),
                    Some(_) => unreachable!(),
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

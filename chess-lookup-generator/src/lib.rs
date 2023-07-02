use chess_bitboard::{BitBoard, File, Pos, Rank};
pub use magic::MagicTable;

mod magic;

pub fn rook_rays(pos: Pos) -> BitBoard {
    (BitBoard::from(pos.rank()) | BitBoard::from(pos.file())) - BitBoard::from(pos)
}

pub fn bishop_rays(pos: Pos) -> BitBoard {
    let mut board = BitBoard::empty();

    let mut a = BitBoard::from_pos(pos);
    let mut b = BitBoard::from_pos(pos);
    let mut c = BitBoard::from_pos(pos);
    let mut d = BitBoard::from_pos(pos);

    loop {
        a = a.shift_up().shift_left();
        b = b.shift_up().shift_right();
        c = c.shift_down().shift_left();
        d = d.shift_down().shift_right();

        let all = a | b | c | d;
        if all.none() {
            break;
        }

        board |= all;
    }

    board
}

pub fn rook_moves() -> magic::MagicTable {
    magic::generate_tables(
        |pos| {
            let rays = rook_rays(pos);
            let mut moves = rays;

            if pos.rank() != Rank::_1 {
                moves -= BitBoard::from(Rank::_1);
            }

            if pos.rank() != Rank::_8 {
                moves -= BitBoard::from(Rank::_8);
            }

            if pos.file() != File::A {
                moves -= BitBoard::from(File::A);
            }

            if pos.file() != File::H {
                moves -= BitBoard::from(File::H);
            }

            (rays, moves)
        },
        |pos, blockers| {
            let mut up = BitBoard::from_pos(pos);
            let mut down = BitBoard::from_pos(pos);
            let mut left = BitBoard::from_pos(pos);
            let mut right = BitBoard::from_pos(pos);

            let mut solution = BitBoard::empty();

            loop {
                up = up.shift_up();
                down = down.shift_down();
                left = left.shift_left();
                right = right.shift_right();

                let all = up | down | left | right;

                if all.none() {
                    break solution;
                }

                solution |= all;

                up -= blockers;
                down -= blockers;
                left -= blockers;
                right -= blockers;
            }
        },
    )
}

pub fn bishop_moves() -> magic::MagicTable {
    let edges = BitBoard::from(Rank::_1)
        | BitBoard::from(Rank::_8)
        | BitBoard::from(File::A)
        | BitBoard::from(File::H);

    magic::generate_tables(
        |pos| {
            let rays = bishop_rays(pos);
            (rays, rays - edges)
        },
        |pos, blockers| {
            let mut a = BitBoard::from_pos(pos);
            let mut b = BitBoard::from_pos(pos);
            let mut c = BitBoard::from_pos(pos);
            let mut d = BitBoard::from_pos(pos);

            let mut solution = BitBoard::empty();

            loop {
                a = a.shift_up().shift_left();
                b = b.shift_up().shift_right();
                c = c.shift_down().shift_left();
                d = d.shift_down().shift_right();

                let all = a | b | c | d;

                if all.none() {
                    break solution;
                }

                solution |= all;

                a -= blockers;
                b -= blockers;
                c -= blockers;
                d -= blockers;
            }
        },
    )
}

pub fn between() -> Vec<BitBoard> {
    let mut boards = Vec::new();

    for a in Pos::all() {
        let a_file = a.file() as i8;
        let a_rank = a.rank() as i8;

        for b in Pos::all() {
            if a == b {
                boards.push(BitBoard::empty());
                continue;
            }

            let b_file = b.file() as i8;
            let b_rank = b.rank() as i8;

            boards.push(if a_file == b_file {
                Rank::all()
                    .filter(|&rank| a.rank() < rank && rank < b.rank())
                    .map(|rank| Pos::new(a.file(), rank))
                    .collect()
            } else if a_rank == b_rank {
                File::all()
                    .filter(|&file| a.file() < file && file < b.file())
                    .map(|file| Pos::new(file, a.rank()))
                    .collect()
            } else {
                let (file, rank, dist) = if a_file < b_file {
                    (a_file as u8, a_rank as u8, b_file - a_file)
                } else {
                    (b_file as u8, b_rank as u8, a_file - b_file)
                };

                'bishop_moves: {
                    let sign = if a_file - b_file == a_rank - b_rank {
                        1
                    } else if a_file - b_file == b_rank - a_rank {
                        -1
                    } else {
                        break 'bishop_moves BitBoard::empty();
                    };

                    (1..8)
                        .take(dist as usize - 1)
                        .map_while(|i| {
                            Some(Pos::new(
                                File::from_u8(file.wrapping_add_signed(i))?,
                                Rank::from_u8(rank.wrapping_add_signed(sign * i))?,
                            ))
                        })
                        .collect()
                }
            });
        }
    }

    boards
}

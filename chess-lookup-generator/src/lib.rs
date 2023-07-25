use chess_bitboard::{BitBoard, File, Pos, Rank};
pub use magic::MagicTable;

pub mod book;
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
                let min_rank = a.rank().min(b.rank());
                let max_rank = a.rank().max(b.rank());

                Rank::all()
                    .filter(|&rank| min_rank < rank && rank < max_rank)
                    .map(|rank| Pos::new(a.file(), rank))
                    .collect()
            } else if a_rank == b_rank {
                let min_file = a.file().min(b.file());
                let max_file = a.file().max(b.file());

                File::all()
                    .filter(|&file| min_file < file && file < max_file)
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

pub fn line() -> Vec<BitBoard> {
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
                Rank::all().map(|rank| Pos::new(a.file(), rank)).collect()
            } else if a_rank == b_rank {
                File::all().map(|file| Pos::new(file, a.rank())).collect()
            } else {
                'bishop_moves: {
                    let sign = if a_file - b_file == a_rank - b_rank {
                        1
                    } else if a_file - b_file == b_rank - a_rank {
                        -1
                    } else {
                        break 'bishop_moves BitBoard::empty();
                    };

                    Pos::all()
                        .filter(|&pos| {
                            let p_file = pos.file() as i8;
                            let p_rank = pos.rank() as i8;

                            (a_file - p_file) * sign == a_rank - p_rank
                        })
                        .collect()
                }
            });
        }
    }

    boards
}

pub fn knight_moves(pos: Pos) -> BitBoard {
    let mut moves = BitBoard::empty();
    let pos_bb = BitBoard::from(pos);

    moves |= pos_bb.shift_up().shift_up().shift_left();
    moves |= pos_bb.shift_up().shift_up().shift_right();
    moves |= pos_bb.shift_down().shift_down().shift_left();
    moves |= pos_bb.shift_down().shift_down().shift_right();

    moves |= pos_bb.shift_left().shift_left().shift_up();
    moves |= pos_bb.shift_left().shift_left().shift_down();
    moves |= pos_bb.shift_right().shift_right().shift_up();
    moves |= pos_bb.shift_right().shift_right().shift_down();

    moves
}

pub fn king_moves(pos: Pos) -> BitBoard {
    let mut moves = BitBoard::empty();
    let pos_bb = BitBoard::from(pos);

    moves |= pos_bb.shift_up();
    moves |= pos_bb.shift_down();
    moves |= pos_bb.shift_left();
    moves |= pos_bb.shift_right();

    moves |= pos_bb.shift_up().shift_left();
    moves |= pos_bb.shift_up().shift_right();
    moves |= pos_bb.shift_down().shift_left();
    moves |= pos_bb.shift_down().shift_right();

    moves
}

pub fn pawn_attacks(pos: Pos) -> [BitBoard; 2] {
    let pos_bb = BitBoard::from(pos);

    let mut white_moves = BitBoard::empty();
    white_moves |= pos_bb.shift_up().shift_left();
    white_moves |= pos_bb.shift_up().shift_right();

    let mut black_moves = BitBoard::empty();
    black_moves |= pos_bb.shift_down().shift_left();
    black_moves |= pos_bb.shift_down().shift_right();

    [white_moves, black_moves]
}

pub fn pawn_quiets(pos: Pos) -> [BitBoard; 2] {
    let pos_bb = BitBoard::from(pos);

    let mut white_moves = BitBoard::empty();
    white_moves |= pos_bb.shift_up();
    if pos.rank() == Rank::_2 {
        white_moves |= pos_bb.shift_up().shift_up();
    }

    let mut black_moves = BitBoard::empty();
    black_moves |= pos_bb.shift_down();
    if pos.rank() == Rank::_7 {
        black_moves |= pos_bb.shift_down().shift_down();
    }

    [white_moves, black_moves]
}

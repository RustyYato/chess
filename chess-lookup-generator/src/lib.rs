use chess_bitboard::{BitBoard, Pos};

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

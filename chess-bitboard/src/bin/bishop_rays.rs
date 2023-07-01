use chess_bitboard::{BitBoard, Pos};

fn main() {
    println!("static ROOK_RAYS: [u64; 64] = [");
    for pos in Pos::all() {
        let board = BitBoard::from_pos(pos);

        let mut rays = board;

        let mut a = board;
        let mut b = board;
        let mut c = board;
        let mut d = board;

        loop {
            let all = a | b | c | d;
            if all.none() {
                break;
            }

            rays |= all;
            a = a.shift_up().shift_left();
            b = b.shift_up().shift_right();
            c = c.shift_down().shift_left();
            d = d.shift_down().shift_right();
        }

        let rays = rays - board;

        println!("    0x{:x},", rays.to_u64());
    }
    println!(
        "];

use chess_bitboard::{{BitBoard, Pos}};

pub fn get(pos: Pos) -> BitBoard {{
    BitBoard::from_u64(ROOK_RAYS[pos as usize])
}}
"
    )
}

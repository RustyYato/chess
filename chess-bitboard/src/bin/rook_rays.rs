use chess_bitboard::{BitBoard, Pos};

fn main() {
    println!("static ROOK_RAYS: [u64; 64] = [");
    for pos in Pos::all() {
        let board = (BitBoard::from(pos.rank()) | BitBoard::from(pos.file())) - BitBoard::from(pos);

        println!("    0x{:x},", board.to_u64());
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

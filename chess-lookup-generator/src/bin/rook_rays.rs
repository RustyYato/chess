use chess_bitboard::Pos;

fn main() {
    println!("static RAYS: [u64; 64] = [");
    for pos in Pos::all() {
        let board = chess_lookup_generator::rook_rays(pos);

        println!("    0x{:x},", board.to_u64());
    }
    println!(
        "];

use chess_bitboard::{{BitBoard, Pos}};

pub fn get(pos: Pos) -> BitBoard {{
    BitBoard::from_u64(RAYS[pos as usize])
}}
"
    )
}

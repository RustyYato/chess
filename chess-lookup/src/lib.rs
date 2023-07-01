use chess_bitboard::{BitBoard, Pos};

mod bishop_rays;
mod rook_rays;

#[inline]
pub fn rook_rays(pos: Pos) -> BitBoard {
    BitBoard::from(rook_rays::RAYS[pos as usize])
}

#[inline]
pub fn bishop_rays(pos: Pos) -> BitBoard {
    BitBoard::from(bishop_rays::RAYS[pos as usize])
}

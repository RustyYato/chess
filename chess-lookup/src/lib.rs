#![forbid(unsafe_op_in_unsafe_fn)]

use chess_bitboard::{BitBoard, Pos};

mod bishop_moves;
mod bishop_rays;
mod rook_moves;
mod rook_rays;

struct Magic {
    mask: u64,
    factor: u64,
    offset: u32,
    shift: u32,
}

#[inline]
pub fn rook_rays(pos: Pos) -> BitBoard {
    BitBoard::from(rook_rays::RAYS[pos as usize])
}

#[inline]
pub fn bishop_rays(pos: Pos) -> BitBoard {
    BitBoard::from(bishop_rays::RAYS[pos as usize])
}

pub fn bishop_moves(pos: Pos, all_pieces: BitBoard) -> BitBoard {
    let magic = &bishop_moves::MOVES_MAGIC[pos as usize];
    let blockers = magic.mask & all_pieces.to_u64();
    let index = blockers.wrapping_mul(magic.factor) >> magic.shift;
    let index = index.wrapping_add(u64::from(magic.offset)) as usize;
    debug_assert!(index < bishop_moves::SOLUTIONS.len());
    if cfg!(debug_assertions) {
        BitBoard::from(bishop_moves::SOLUTIONS[index])
    } else {
        BitBoard::from(unsafe { *bishop_moves::SOLUTIONS.get_unchecked(index) })
    }
}

pub fn rook_moves(pos: Pos, all_pieces: BitBoard) -> BitBoard {
    let magic = &rook_moves::MOVES_MAGIC[pos as usize];
    let blockers = magic.mask & all_pieces.to_u64();
    let index = blockers.wrapping_mul(magic.factor) >> magic.shift;
    let index = index.wrapping_add(u64::from(magic.offset)) as usize;
    debug_assert!(index < rook_moves::SOLUTIONS.len());
    if cfg!(debug_assertions) {
        BitBoard::from(rook_moves::SOLUTIONS[index])
    } else {
        BitBoard::from(unsafe { *rook_moves::SOLUTIONS.get_unchecked(index) })
    }
}

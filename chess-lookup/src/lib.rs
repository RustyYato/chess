#![forbid(unsafe_op_in_unsafe_fn)]

use chess_bitboard::{BitBoard, Color, Pos};

mod between;
mod bishop_moves;
mod bishop_rays;
mod knight_moves;
mod pawn_attacks;
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
    BitBoard::from(rook_rays::RAYS[pos])
}

#[inline]
pub fn bishop_rays(pos: Pos) -> BitBoard {
    BitBoard::from(bishop_rays::RAYS[pos])
}

#[inline]
pub fn knight_moves(pos: Pos) -> BitBoard {
    BitBoard::from(knight_moves::MOVES[pos])
}

#[inline]
pub fn pawn_attacks_moves(pos: Pos, color: Color) -> BitBoard {
    BitBoard::from(pawn_attacks::PAWN_ATTACKS[pos][color])
}

#[inline]
pub fn bishop_moves(pos: Pos, all_pieces: BitBoard) -> BitBoard {
    let magic = &bishop_moves::MOVES_MAGIC[pos];
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

#[inline]
pub fn rook_moves(pos: Pos, all_pieces: BitBoard) -> BitBoard {
    let magic = &rook_moves::MOVES_MAGIC[pos];
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

#[inline]
pub fn between(a: Pos, b: Pos) -> BitBoard {
    BitBoard::from(between::SOLUTIONS[a as usize][b as usize])
}

pub const PAWN_DOUBLE_SOURCE: BitBoard =
    BitBoard::from_rank(chess_bitboard::Rank::_2).or(BitBoard::from_rank(chess_bitboard::Rank::_7));

pub const PAWN_DOUBLE_DEST: BitBoard =
    BitBoard::from_rank(chess_bitboard::Rank::_4).or(BitBoard::from_rank(chess_bitboard::Rank::_5));

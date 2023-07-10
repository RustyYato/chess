#![forbid(unsafe_op_in_unsafe_fn)]

use chess_bitboard::{BitBoard, Color, File, Piece, Pos, Rank};

mod between;
mod bishop_moves;
mod bishop_rays;
mod knight_moves;
mod pawn_attacks;
mod rook_moves;
mod rook_rays;
mod zobrist;

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

#[inline]
pub fn zobrist(pos: Pos, piece: Piece, color: Color) -> u64 {
    zobrist::PIECE_ZOBRIST[color][pos][piece]
}

#[inline]
pub fn castle_rights_zobrist(castle_rights: usize) -> u64 {
    zobrist::CASTLE_ZOBRIST[castle_rights]
}

#[inline]
pub fn en_passant_zobrist(file: File) -> u64 {
    zobrist::EN_PASSANT_ZOBRIST[file]
}

#[inline]
pub fn turn_zobrist(color: Color) -> u64 {
    zobrist::TURN_ZOBRIST[color]
}

pub const PAWN_DOUBLE_SOURCE: BitBoard =
    BitBoard::from_rank(chess_bitboard::Rank::_2).or(BitBoard::from_rank(chess_bitboard::Rank::_7));

pub const PAWN_DOUBLE_DEST: BitBoard =
    BitBoard::from_rank(chess_bitboard::Rank::_4).or(BitBoard::from_rank(chess_bitboard::Rank::_5));

pub const BACKRANK: [Rank; 2] = [Rank::_1, Rank::_8];
pub const BACKRANK_BB: [BitBoard; 2] =
    [BitBoard::from_rank(Rank::_1), BitBoard::from_rank(Rank::_8)];

pub const CASTLE_MOVES: BitBoard = BitBoard::empty()
    .with(Pos::C1)
    .with(Pos::C8)
    .with(Pos::E1)
    .with(Pos::E8)
    .with(Pos::G1)
    .with(Pos::G8);

pub const PAWN_DOUBLE_MOVE: [BitBoard; 2] = [
    BitBoard::from_rank(Rank::_2).or(BitBoard::from_rank(Rank::_4)),
    BitBoard::from_rank(Rank::_5).or(BitBoard::from_rank(Rank::_7)),
];

pub const ROOK_CASTLE_QUEENSIDE: BitBoard =
    BitBoard::from_file(File::A).or(BitBoard::from_file(File::D));
pub const ROOK_CASTLE_KINGSIDE: BitBoard =
    BitBoard::from_file(File::H).or(BitBoard::from_file(File::F));

pub const CASTLE_ROOK_START: [File; 8] = [
    File::A,
    File::A,
    File::A,
    File::A,
    File::H,
    File::H,
    File::H,
    File::H,
];
pub const CASTLE_ROOK_END: [File; 8] = [
    File::D,
    File::D,
    File::D,
    File::D,
    File::F,
    File::F,
    File::F,
    File::F,
];

pub const PROMOTION_RANK: [Rank; 2] = [Rank::_8, Rank::_1];

pub const PAWN_DOUBLE_MOVE_SOURCE_RANK: [Rank; 2] = [Rank::_2, Rank::_7];
pub const PAWN_DOUBLE_MOVE_DEST_RANK: [Rank; 2] = [Rank::_4, Rank::_5];

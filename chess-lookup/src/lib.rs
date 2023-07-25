#![forbid(unsafe_op_in_unsafe_fn)]

use chess_bitboard::{BitBoard, Color, File, Piece, Pos, Rank};

mod between;
mod bishop_moves;
mod bishop_rays;
mod book;
mod king_moves;
mod knight_moves;
mod line;
mod pawn;
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
pub fn king_moves(pos: Pos) -> BitBoard {
    BitBoard::from(king_moves::MOVES[pos])
}

#[inline]
pub fn pawn_moves(pos: Pos, color: Color, all_pieces: BitBoard) -> BitBoard {
    pawn_quiets(pos, color, all_pieces) | pawn_attacks(pos, color, all_pieces)
}

#[inline]
pub fn pawn_quiets(pos: Pos, color: Color, all_pieces: BitBoard) -> BitBoard {
    let current = BitBoard::from(pos);
    let next = match color {
        Color::White => current.shift_up(),
        Color::Black => current.shift_down(),
    };
    if (next & all_pieces).any() {
        BitBoard::empty()
    } else {
        BitBoard::from(pawn::PAWN_QUIETS[pos][color]) & !all_pieces
    }
}

#[inline]
pub fn pawn_attacks(pos: Pos, color: Color, all_pieces: BitBoard) -> BitBoard {
    BitBoard::from(pawn::PAWN_ATTACKS[pos][color]) & all_pieces
}

#[inline]
pub fn pawn_attacks_moves(pos: Pos, color: Color) -> BitBoard {
    BitBoard::from(pawn::PAWN_ATTACKS[pos][color])
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
pub fn distance(a: Pos, b: Pos) -> u8 {
    let a_rank = a.rank() as u8;
    let b_rank = b.rank() as u8;

    let a_file = a.file() as u8;
    let b_file = b.file() as u8;

    a_rank.abs_diff(b_rank).max(a_file.abs_diff(b_file))
}

#[inline]
pub fn line(a: Pos, b: Pos) -> BitBoard {
    BitBoard::from(line::SOLUTIONS[a as usize][b as usize])
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

pub static ADJACENT_FILES: [BitBoard; 8] = {
    let mut files = [BitBoard::empty(); 8];

    let mut i = 0;

    while i < 8 {
        let board = BitBoard::from_file(File::const_from_u8(i as u8));
        files[i] = board.shift_left().or(board.shift_right());
        i += 1;
    }

    files
};

pub static ADJACENT_RANKS: [BitBoard; 8] = {
    let mut ranks = [BitBoard::empty(); 8];

    let mut i = 0;

    while i < 8 {
        let board = BitBoard::from_rank(Rank::const_from_u8(i as u8));
        ranks[i] = board.shift_up().or(board.shift_down());
        i += 1;
    }

    ranks
};

pub const KINGSIDE_CASTLE_FILES: BitBoard =
    BitBoard::from_file(File::F).or(BitBoard::from_file(File::G));
pub const QUEENSIDE_CASTLE_FILES: BitBoard = BitBoard::from_file(File::B)
    .or(BitBoard::from_file(File::C))
    .or(BitBoard::from_file(File::D));

pub const KINGSIDE_CASTLE_SAFE_FILES: BitBoard =
    BitBoard::from_file(File::F).or(BitBoard::from_file(File::G));
pub const QUEENSIDE_CASTLE_SAFE_FILES: BitBoard =
    BitBoard::from_file(File::C).or(BitBoard::from_file(File::D));

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct BookMoves {
    index: usize,
}

impl core::fmt::Debug for BookMoves {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "book{}", self.index)
    }
}

pub const INITIAL_BOOOK_MOVES: BookMoves = BookMoves {
    index: book::BOOK_SIZE - 1,
};
pub const EMPTY_BOOK_MOVES: BookMoves = BookMoves { index: 0 };

impl IntoIterator for BookMoves {
    type Item = BookMove;
    type IntoIter = BookMovesIter;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        BookMovesIter { index: self.index }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BookMove {
    pub children: BookMoves,
    pub source: Pos,
    pub dest: Pos,
}

#[derive(Clone)]
pub struct BookMovesIter {
    index: usize,
}

impl Iterator for BookMovesIter {
    type Item = BookMove;

    fn next(&mut self) -> Option<Self::Item> {
        let offset = unsafe { *book::BOOK.get_unchecked(self.index) } as usize;
        if offset == 0 {
            return None;
        }

        let mv = unsafe { *book::BOOK.get_unchecked(self.index - 1) } as usize;
        let child_index = self.index - 2;
        self.index = self.index.checked_sub(offset + 1)?;

        let source = Pos::from_u8((mv & 0x3f) as u8).unwrap();
        let dest = Pos::from_u8(((mv >> 6) & 0x3f) as u8).unwrap();

        Some(BookMove {
            source,
            dest,
            children: BookMoves { index: child_index },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_book_indices() {
        fn walk(book_moves: BookMoves) {
            for x in book_moves {
                walk(x.children);
            }
        }

        walk(INITIAL_BOOOK_MOVES);
    }
}

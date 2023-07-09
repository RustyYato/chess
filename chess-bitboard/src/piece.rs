use core::ops::{Index, IndexMut, Range};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PromotionPiece {
    Knight = Piece::Knight as u8,
    Bishop = Piece::Bishop as u8,
    Rook = Piece::Rook as u8,
    Queen = Piece::Queen as u8,
}

impl From<PromotionPiece> for Piece {
    #[inline]
    fn from(promotion_piece: PromotionPiece) -> Self {
        promotion_piece.to_piece()
    }
}

impl<T> Index<Piece> for [T; 6] {
    type Output = T;

    #[inline]
    fn index(&self, index: Piece) -> &Self::Output {
        &self[index as usize]
    }
}

impl<T> IndexMut<Piece> for [T; 6] {
    #[inline]
    fn index_mut(&mut self, index: Piece) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

impl Piece {
    #[inline]
    pub fn from_u8(index: u8) -> Option<Self> {
        Some(match index {
            0 => Self::Pawn,
            1 => Self::Knight,
            2 => Self::Bishop,
            3 => Self::Rook,
            4 => Self::Queen,
            5 => Self::King,
            6.. => return None,
        })
    }

    #[inline]
    pub const fn all() -> AllPieceIter {
        AllPieceIter { range: 0..6 }
    }
}

impl PromotionPiece {
    #[inline]
    pub const fn to_piece(self) -> Piece {
        match self {
            PromotionPiece::Knight => Piece::Knight,
            PromotionPiece::Bishop => Piece::Bishop,
            PromotionPiece::Rook => Piece::Rook,
            PromotionPiece::Queen => Piece::Queen,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AllPieceIter {
    range: Range<u8>,
}

impl Iterator for AllPieceIter {
    type Item = Piece;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        Some(Piece::from_u8(self.range.next()?).unwrap())
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        Some(Piece::from_u8(self.range.nth(n)?).unwrap())
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.range.size_hint()
    }
}

impl DoubleEndedIterator for AllPieceIter {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        Some(Piece::from_u8(self.range.next_back()?).unwrap())
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        Some(Piece::from_u8(self.range.nth_back(n)?).unwrap())
    }
}

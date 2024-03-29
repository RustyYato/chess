use core::{
    fmt::{Display, Write},
    ops::{Index, IndexMut, Range},
    str::FromStr,
};

#[repr(u8)]
#[cfg_attr(feature = "abi_stable", derive(abi_stable::StableAbi))]
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
#[cfg_attr(feature = "abi_stable", derive(abi_stable::StableAbi))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

impl Display for PromotionPiece {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let c = match self {
            PromotionPiece::Knight => 'N',
            PromotionPiece::Bishop => 'B',
            PromotionPiece::Rook => 'R',
            PromotionPiece::Queen => 'Q',
        };

        f.write_char(c)
    }
}

impl FromStr for Piece {
    type Err = ();

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_ascii_bytes(s.as_bytes()).ok_or(())
    }
}

impl Piece {
    #[inline]
    pub fn from_ascii_bytes(s: &[u8]) -> Option<Self> {
        let &[s] = s else { return None };

        Self::from_ascii_byte(s)
    }

    #[inline]
    pub fn from_ascii_byte(s: u8) -> Option<Self> {
        Some(match s {
            b'p' | b'P' => Self::Pawn,
            b'n' | b'N' => Self::Knight,
            b'b' | b'B' => Self::Bishop,
            b'r' | b'R' => Self::Rook,
            b'q' | b'Q' => Self::Queen,
            b'k' | b'K' => Self::King,
            _ => return None,
        })
    }
}

impl FromStr for PromotionPiece {
    type Err = ();

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_ascii_bytes(s.as_bytes()).ok_or(())
    }
}

impl PromotionPiece {
    #[inline]
    pub fn from_ascii_bytes(s: &[u8]) -> Option<Self> {
        let &[s] = s else { return None };

        Self::from_ascii_byte(s)
    }

    #[inline]
    pub fn from_ascii_byte(s: u8) -> Option<Self> {
        Some(match s {
            b'n' | b'N' => Self::Knight,
            b'b' | b'B' => Self::Bishop,
            b'r' | b'R' => Self::Rook,
            b'q' | b'Q' => Self::Queen,
            _ => return None,
        })
    }
}

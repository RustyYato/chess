use core::ops::{Index, IndexMut, Not, Range};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Side {
    King,
    Queen,
}

impl<T> Index<Side> for [T; 2] {
    type Output = T;

    #[inline]
    fn index(&self, index: Side) -> &Self::Output {
        &self[index as usize]
    }
}

impl<T> IndexMut<Side> for [T; 2] {
    #[inline]
    fn index_mut(&mut self, index: Side) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

impl Not for Side {
    type Output = Self;

    #[inline]
    fn not(self) -> Self::Output {
        match self {
            Side::King => Side::Queen,
            Side::Queen => Side::King,
        }
    }
}

impl Side {
    #[inline]
    pub fn from_u8(index: u8) -> Option<Self> {
        Some(match index {
            0 => Self::King,
            1 => Self::Queen,
            2.. => return None,
        })
    }

    pub const fn all() -> AllSideIter {
        AllSideIter { range: 0..2 }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AllSideIter {
    range: Range<u8>,
}

impl Iterator for AllSideIter {
    type Item = Side;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        Some(Side::from_u8(self.range.next()?).unwrap())
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        Some(Side::from_u8(self.range.nth(n)?).unwrap())
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.range.size_hint()
    }
}

impl DoubleEndedIterator for AllSideIter {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        Some(Side::from_u8(self.range.next_back()?).unwrap())
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        Some(Side::from_u8(self.range.nth_back(n)?).unwrap())
    }
}

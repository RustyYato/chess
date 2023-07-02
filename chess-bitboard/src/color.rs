use core::ops::{Index, IndexMut, Not, Range};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    White,
    Black,
}

impl<T> Index<Color> for [T; 2] {
    type Output = T;

    #[inline]
    fn index(&self, index: Color) -> &Self::Output {
        &self[index as usize]
    }
}

impl<T> IndexMut<Color> for [T; 2] {
    #[inline]
    fn index_mut(&mut self, index: Color) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

impl Not for Color {
    type Output = Self;

    #[inline]
    fn not(self) -> Self::Output {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

impl Color {
    #[inline]
    pub fn from_u8(index: u8) -> Option<Self> {
        Some(match index {
            0 => Self::White,
            1 => Self::Black,
            2.. => return None,
        })
    }

    pub const fn all() -> AllColorIter {
        AllColorIter { range: 0..6 }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AllColorIter {
    range: Range<u8>,
}

impl Iterator for AllColorIter {
    type Item = Color;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        Some(Color::from_u8(self.range.next()?).unwrap())
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        Some(Color::from_u8(self.range.nth(n)?).unwrap())
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.range.size_hint()
    }
}

impl DoubleEndedIterator for AllColorIter {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        Some(Color::from_u8(self.range.next_back()?).unwrap())
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        Some(Color::from_u8(self.range.nth_back(n)?).unwrap())
    }
}

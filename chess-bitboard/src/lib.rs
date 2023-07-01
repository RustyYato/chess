#![no_std]

mod fmt;
mod ops;
mod pos;

use core::num::NonZeroU64;

pub use pos::{File, Pos, Rank};

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct BitBoard(u64);

impl BitBoard {
    #[inline(always)]
    pub const fn to_u64(self) -> u64 {
        self.0
    }

    #[inline(always)]
    pub const fn from_u64(board: u64) -> Self {
        Self(board)
    }

    #[inline(always)]
    pub const fn empty() -> Self {
        Self(0)
    }

    #[inline(always)]
    pub const fn from_pos(pos: Pos) -> Self {
        Self(1 << pos as u8)
    }

    #[inline(always)]
    pub const fn from_file(file: File) -> Self {
        const FIRST_FILE: u64 = 0xff;
        Self(FIRST_FILE << (file as u8 * 8))
    }

    #[inline(always)]
    pub const fn from_rank(rank: Rank) -> Self {
        const FIRST_RANK: u64 = 0x0101010101010101;
        Self(FIRST_RANK << rank as u8)
    }

    #[inline(always)]
    pub const fn contains(self, pos: Pos) -> bool {
        self.and(BitBoard::from_pos(pos)).any()
    }

    #[inline(always)]
    pub const fn with(self, pos: Pos) -> Self {
        self.or(Self::from_pos(pos))
    }

    #[inline(always)]
    pub const fn cleared(self, pos: Pos) -> Self {
        self.diff(Self::from_pos(pos))
    }

    #[inline(always)]
    pub fn set(&mut self, pos: Pos) {
        *self |= Self::from_pos(pos)
    }

    #[inline(always)]
    pub fn clear(&mut self, pos: Pos) {
        *self -= Self::from_pos(pos)
    }

    #[inline(always)]
    pub const fn or(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    #[inline(always)]
    pub const fn and(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }

    #[inline(always)]
    pub const fn xor(self, other: Self) -> Self {
        Self(self.0 ^ other.0)
    }

    #[inline(always)]
    pub const fn not(self) -> Self {
        Self(!self.0)
    }

    #[inline(always)]
    pub const fn diff(self, other: Self) -> Self {
        self.and(other.not())
    }

    #[inline(always)]
    pub const fn shift_up(self) -> Self {
        let board = self.diff(Self::from_rank(Rank::_8));
        Self(board.0 << 1)
    }

    #[inline(always)]
    pub const fn shift_down(self) -> Self {
        let board = self.diff(Self::from_rank(Rank::_1));
        Self(board.0 >> 1)
    }

    #[inline(always)]
    pub const fn shift_left(self) -> Self {
        let board = self.diff(Self::from_file(File::A));
        Self(board.0 >> 8)
    }

    #[inline(always)]
    pub const fn shift_right(self) -> Self {
        let board = self.diff(Self::from_file(File::H));
        Self(board.0 << 8)
    }

    #[inline(always)]
    pub fn pop(&mut self) -> Option<Pos> {
        let pos = NonZeroU64::new(self.0)?;
        let zeros = pos.trailing_zeros() as u8;
        let pos = Pos::from_u8(zeros).unwrap();
        *self ^= BitBoard::from_pos(pos);
        Some(pos)
    }

    #[inline(always)]
    pub const fn iter(self) -> BitBoardIter {
        BitBoardIter(self)
    }

    #[inline(always)]
    pub const fn count(self) -> u8 {
        self.0.count_ones() as u8
    }

    #[inline(always)]
    pub const fn any(self) -> bool {
        self.0 != 0
    }

    #[inline(always)]
    pub const fn none(self) -> bool {
        self.0 == 0
    }

    #[inline(always)]
    pub const fn all(self) -> bool {
        self.not().none()
    }

    #[inline(always)]
    pub const fn some(self) -> bool {
        self.not().any()
    }
}

#[repr(transparent)]
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct BitBoardIter(BitBoard);

impl Iterator for BitBoardIter {
    type Item = Pos;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = usize::from(self.0.count());

        (remaining, Some(remaining))
    }

    #[cfg(target_feature = "bmi2")]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let x = unsafe { core::arch::x86_64::_pdep_u64(1 << n, self.0.to_u64()) }.trailing_zeros()
            as u8;
        let pos = Pos::from_u8(x)?;
        let mask = (1 << (1 + pos as u32)) - 1;
        self.0 -= BitBoard::from(mask);
        Some(pos)
    }
}

impl IntoIterator for BitBoard {
    type Item = Pos;
    type IntoIter = BitBoardIter;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl FromIterator<Pos> for BitBoard {
    fn from_iter<T: IntoIterator<Item = Pos>>(iter: T) -> Self {
        let mut board = BitBoard::empty();
        iter.into_iter().for_each(|pos| board.set(pos));
        board
    }
}

impl From<Pos> for BitBoard {
    #[inline]
    fn from(value: Pos) -> Self {
        BitBoard::from_pos(value)
    }
}

impl From<File> for BitBoard {
    #[inline]
    fn from(value: File) -> Self {
        BitBoard::from_file(value)
    }
}

impl From<Rank> for BitBoard {
    #[inline]
    fn from(value: Rank) -> Self {
        BitBoard::from_rank(value)
    }
}

impl From<u64> for BitBoard {
    #[inline]
    fn from(value: u64) -> Self {
        BitBoard::from_u64(value)
    }
}

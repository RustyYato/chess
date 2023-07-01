#![no_std]
#![forbid(unsafe_code)]

mod fmt;
mod ops;
mod pos;

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
}

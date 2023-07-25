use core::{ops::Range, str::FromStr};

use crate::Side;

#[repr(u8)]
#[cfg_attr(feature = "abi_stable", derive(abi_stable::StableAbi))]
#[rustfmt::skip]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Pos {
    A1, B1, C1, D1, E1, F1, G1, H1, 
    A2, B2, C2, D2, E2, F2, G2, H2, 
    A3, B3, C3, D3, E3, F3, G3, H3, 
    A4, B4, C4, D4, E4, F4, G4, H4, 
    A5, B5, C5, D5, E5, F5, G5, H5, 
    A6, B6, C6, D6, E6, F6, G6, H6, 
    A7, B7, C7, D7, E7, F7, G7, H7, 
    A8, B8, C8, D8, E8, F8, G8, H8, 

}

#[repr(u8)]
#[cfg_attr(feature = "abi_stable", derive(abi_stable::StableAbi))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

#[repr(u8)]
#[cfg_attr(feature = "abi_stable", derive(abi_stable::StableAbi))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Rank {
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
}

impl Pos {
    #[inline(always)]
    pub const fn new(file: File, rank: Rank) -> Self {
        match Self::from_u8(rank as u8 * 8 + file as u8) {
            Some(x) => x,
            None => unreachable!(),
        }
    }

    #[inline]
    #[rustfmt::skip]
    pub const fn from_u8(pos: u8) -> Option<Self> {
        use Pos::*;
        Some(match pos {
            0 => A1, 1 => B1, 2 => C1, 3 => D1, 4 => E1, 5 => F1, 6 => G1, 7 => H1, 
            8 => A2, 9 => B2, 10 => C2, 11 => D2, 12 => E2, 13 => F2, 14 => G2, 15 => H2, 
            16 => A3, 17 => B3, 18 => C3, 19 => D3, 20 => E3, 21 => F3, 22 => G3, 23 => H3, 
            24 => A4, 25 => B4, 26 => C4, 27 => D4, 28 => E4, 29 => F4, 30 => G4, 31 => H4, 
            32 => A5, 33 => B5, 34 => C5, 35 => D5, 36 => E5, 37 => F5, 38 => G5, 39 => H5, 
            40 => A6, 41 => B6, 42 => C6, 43 => D6, 44 => E6, 45 => F6, 46 => G6, 47 => H6, 
            48 => A7, 49 => B7, 50 => C7, 51 => D7, 52 => E7, 53 => F7, 54 => G7, 55 => H7, 
            56 => A8, 57 => B8, 58 => C8, 59 => D8, 60 => E8, 61 => F8, 62 => G8, 63 => H8, 
            64.. => return None
        })
    }

    #[inline(always)]
    pub const fn file(self) -> File {
        match File::from_u8(self as u8 % 8) {
            Some(rank) => rank,
            None => unreachable!(),
        }
    }

    #[inline(always)]
    pub const fn rank(self) -> Rank {
        match Rank::from_u8(self as u8 / 8) {
            Some(rank) => rank,
            None => unreachable!(),
        }
    }

    #[inline]
    pub const fn to_u8(self) -> u8 {
        self as u8
    }

    #[inline]
    pub const fn all() -> AllPosIter {
        AllPosIter { pos: 0 }
    }
}

impl File {
    #[inline(always)]
    pub const fn from_u8(pos: u8) -> Option<Self> {
        Some(match pos {
            0 => File::A,
            1 => File::B,
            2 => File::C,
            3 => File::D,
            4 => File::E,
            5 => File::F,
            6 => File::G,
            7 => File::H,
            8.. => return None,
        })
    }

    #[inline(always)]
    pub const fn lower_letter(self) -> char {
        (b'a' + self as u8) as char
    }

    #[inline(always)]
    pub const fn upper_letter(self) -> char {
        (b'A' + self as u8) as char
    }

    #[inline(always)]
    pub const fn to_u8(self) -> u8 {
        self as u8
    }

    #[inline(always)]
    pub const fn dist_to(self, other: Self) -> u8 {
        (self as u8).abs_diff(other as u8)
    }

    #[inline]
    pub const fn all() -> AllFileIter {
        AllFileIter { range: 0..8 }
    }

    #[inline]
    pub const fn iter(self) -> FileIter {
        FileIter {
            file: self,
            ranks: Rank::all(),
        }
    }

    #[inline(always)]
    pub const fn side(self) -> Side {
        if (self as u8) < (File::E as u8) {
            Side::Queen
        } else {
            Side::King
        }
    }
}

impl Rank {
    #[inline(always)]
    pub const fn from_u8(pos: u8) -> Option<Self> {
        Some(match pos {
            0 => Rank::_1,
            1 => Rank::_2,
            2 => Rank::_3,
            3 => Rank::_4,
            4 => Rank::_5,
            5 => Rank::_6,
            6 => Rank::_7,
            7 => Rank::_8,
            _ => return None,
        })
    }

    #[inline(always)]
    pub const fn to_u8(self) -> u8 {
        self as u8
    }

    #[inline(always)]
    pub const fn dist_to(self, other: Self) -> u8 {
        (self as u8).abs_diff(other as u8)
    }

    #[inline]
    pub const fn all() -> AllRankIter {
        AllRankIter { range: 0..8 }
    }

    #[inline]
    pub const fn iter(self) -> RankIter {
        RankIter {
            rank: self,
            files: File::all(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AllPosIter {
    pos: u8,
}

impl Iterator for AllPosIter {
    type Item = Pos;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let pos = Pos::from_u8(self.pos)?;
        self.pos += 1;
        Some(pos)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = usize::from(64 - self.pos);
        (remaining, Some(remaining))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AllFileIter {
    range: Range<u8>,
}

impl AllFileIter {
    fn next_with(&mut self, f: impl FnOnce(&mut Range<u8>) -> Option<u8>) -> Option<File> {
        let rank = File::from_u8(f(&mut self.range)?);
        Some(unsafe { rank.unwrap_unchecked() })
    }
}

impl Iterator for AllFileIter {
    type Item = File;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.next_with(Iterator::next)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.next_with(|range| range.nth(n))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.range.size_hint()
    }
}

impl DoubleEndedIterator for AllFileIter {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.next_with(DoubleEndedIterator::next_back)
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.next_with(|range| range.nth_back(n))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AllRankIter {
    range: Range<u8>,
}

impl AllRankIter {
    fn next_with(&mut self, f: impl FnOnce(&mut Range<u8>) -> Option<u8>) -> Option<Rank> {
        let rank = Rank::from_u8(f(&mut self.range)?);
        Some(unsafe { rank.unwrap_unchecked() })
    }
}

impl Iterator for AllRankIter {
    type Item = Rank;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.next_with(Iterator::next)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.next_with(|range| range.nth(n))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.range.size_hint()
    }
}

impl DoubleEndedIterator for AllRankIter {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.next_with(DoubleEndedIterator::next_back)
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.next_with(|range| range.nth_back(n))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileIter {
    file: File,
    ranks: AllRankIter,
}

impl Iterator for FileIter {
    type Item = Pos;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let rank = self.ranks.next()?;
        Some(Pos::new(self.file, rank))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.ranks.size_hint()
    }
}

impl IntoIterator for File {
    type Item = Pos;
    type IntoIter = FileIter;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RankIter {
    rank: Rank,
    files: AllFileIter,
}

impl Iterator for RankIter {
    type Item = Pos;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let file = self.files.next()?;
        Some(Pos::new(file, self.rank))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.files.size_hint()
    }
}

impl IntoIterator for Rank {
    type Item = Pos;
    type IntoIter = RankIter;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T> core::ops::Index<Pos> for [T; 64] {
    type Output = T;

    #[inline]
    fn index(&self, index: Pos) -> &Self::Output {
        &self[index as usize]
    }
}

impl<T> core::ops::IndexMut<Pos> for [T; 64] {
    #[inline]
    fn index_mut(&mut self, index: Pos) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

impl<T> core::ops::Index<File> for [T; 8] {
    type Output = T;

    #[inline]
    fn index(&self, index: File) -> &Self::Output {
        &self[index as usize]
    }
}

impl<T> core::ops::IndexMut<File> for [T; 8] {
    #[inline]
    fn index_mut(&mut self, index: File) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

impl<T> core::ops::Index<Rank> for [T; 8] {
    type Output = T;

    #[inline]
    fn index(&self, index: Rank) -> &Self::Output {
        &self[index as usize]
    }
}

impl<T> core::ops::IndexMut<Rank> for [T; 8] {
    #[inline]
    fn index_mut(&mut self, index: Rank) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

impl core::fmt::Display for Pos {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}{}", self.file(), self.rank())
    }
}

impl core::fmt::Display for File {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.lower_letter())
    }
}

impl core::fmt::Display for Rank {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", *self as u8 + 1)
    }
}

impl FromStr for Pos {
    type Err = ();

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_ascii_bytes(s.as_bytes()).ok_or(())
    }
}

impl Pos {
    #[inline]
    pub fn from_ascii_bytes(s: &[u8]) -> Option<Self> {
        extern crate std;
        match s {
            &[f, r] => Some(Self::new(
                File::from_ascii_byte(f)?,
                Rank::from_ascii_byte(r)?,
            )),
            _ => None,
        }
    }
}

impl FromStr for File {
    type Err = ();

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_ascii_bytes(s.as_bytes()).ok_or(())
    }
}

impl File {
    #[inline]
    pub fn from_ascii_bytes(s: &[u8]) -> Option<Self> {
        let &[s] = s else {
            return None
        };

        Self::from_ascii_byte(s)
    }

    #[inline]
    pub fn from_ascii_byte(s: u8) -> Option<Self> {
        let s = (s | 0b0010_0000).wrapping_sub(b'a');
        Self::from_u8(s)
    }
}

impl FromStr for Rank {
    type Err = ();

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_ascii_bytes(s.as_bytes()).ok_or(())
    }
}

impl Rank {
    #[inline]
    pub fn from_ascii_bytes(s: &[u8]) -> Option<Self> {
        let &[s] = s else {
            return None
        };

        Self::from_ascii_byte(s)
    }

    #[inline]
    pub fn from_ascii_byte(s: u8) -> Option<Self> {
        let s = s.wrapping_sub(b'1');
        Self::from_u8(s)
    }
}

#[test]
fn test_ascii_byte() {
    for i in b'a'..=b'h' {
        let file = File::from_u8(i - b'a').unwrap();
        assert_eq!(file, File::from_ascii_byte(i).unwrap())
    }
}

#[test]
fn test_file_and_rank() {
    for pos in Pos::all() {
        assert_eq!(Pos::new(pos.file(), pos.rank()), pos);
    }
}

#[test]
fn test_file_and_rank_into_pos() {
    for file in File::all() {
        for rank in Rank::all() {
            let pos = Pos::new(file, rank);
            assert_eq!(pos.file(), file);
            assert_eq!(pos.rank(), rank);
        }
    }
}

#[test]
fn test_from_u8() {
    for pos in Pos::all() {
        assert_eq!(Pos::from_u8(pos as u8), Some(pos));
    }
}

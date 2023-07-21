use core::{ops::Range, str::FromStr};

use crate::Side;

#[repr(u8)]
#[rustfmt::skip]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Pos {
    A1,    A2,    A3,    A4,    A5,    A6,    A7,    A8,
    B1,    B2,    B3,    B4,    B5,    B6,    B7,    B8,
    C1,    C2,    C3,    C4,    C5,    C6,    C7,    C8,
    D1,    D2,    D3,    D4,    D5,    D6,    D7,    D8,
    E1,    E2,    E3,    E4,    E5,    E6,    E7,    E8,
    F1,    F2,    F3,    F4,    F5,    F6,    F7,    F8,
    G1,    G2,    G3,    G4,    G5,    G6,    G7,    G8,
    H1,    H2,    H3,    H4,    H5,    H6,    H7,    H8,
}

#[repr(u8)]
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
        match Self::from_u8(file as u8 * 8 + rank as u8) {
            Some(x) => x,
            None => unreachable!(),
        }
    }

    #[inline]
    #[rustfmt::skip]
    pub const fn from_u8(pos: u8) -> Option<Self> {
        Some(match pos {
            0 => Pos::A1,    1 => Pos::A2,    2 => Pos::A3,    3 => Pos::A4,    4 => Pos::A5,    5 => Pos::A6,    6 => Pos::A7,    7 => Pos::A8,
            8 => Pos::B1,    9 => Pos::B2,    10 => Pos::B3,    11 => Pos::B4,    12 => Pos::B5,    13 => Pos::B6,    14 => Pos::B7,    15 => Pos::B8,
            16 => Pos::C1,    17 => Pos::C2,    18 => Pos::C3,    19 => Pos::C4,    20 => Pos::C5,    21 => Pos::C6,    22 => Pos::C7,    23 => Pos::C8,
            24 => Pos::D1,    25 => Pos::D2,    26 => Pos::D3,    27 => Pos::D4,    28 => Pos::D5,    29 => Pos::D6,    30 => Pos::D7,    31 => Pos::D8,
            32 => Pos::E1,    33 => Pos::E2,    34 => Pos::E3,    35 => Pos::E4,    36 => Pos::E5,    37 => Pos::E6,    38 => Pos::E7,    39 => Pos::E8,
            40 => Pos::F1,    41 => Pos::F2,    42 => Pos::F3,    43 => Pos::F4,    44 => Pos::F5,    45 => Pos::F6,    46 => Pos::F7,    47 => Pos::F8,
            48 => Pos::G1,    49 => Pos::G2,    50 => Pos::G3,    51 => Pos::G4,    52 => Pos::G5,    53 => Pos::G6,    54 => Pos::G7,    55 => Pos::G8,
            56 => Pos::H1,    57 => Pos::H2,    58 => Pos::H3,    59 => Pos::H4,    60 => Pos::H5,    61 => Pos::H6,    62 => Pos::H7,    63 => Pos::H8,
            64.. => return None
        })        
    }

    #[inline(always)]
    pub const fn file(self) -> File {
        match File::from_u8(self as u8 / 8) {
            Some(rank) => rank,
            None => unreachable!(),
        }
    }

    #[inline(always)]
    pub const fn rank(self) -> Rank {
        match Rank::from_u8(self as u8 % 8) {
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
            8.. => return None
        })
    }

    #[inline(always)]
    pub const fn lower_letter(self) -> char {
        (b'a' + self as u8 ) as char
    }

    #[inline(always)]
    pub const fn upper_letter(self) -> char {
        (b'A' + self as u8 ) as char
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
        FileIter { file: self, ranks: Rank::all() }
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
            _ => return None
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
        RankIter { rank: self, files: File::all() }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AllPosIter {
    pos: u8
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
    range: Range<u8>
}

impl  AllFileIter {
    fn next_with(&mut self, f: impl FnOnce(&mut Range<u8>) -> Option<u8>) -> Option<File> {
        let rank = File::from_u8(f(&mut self.range)?);
        Some(unsafe{ rank.unwrap_unchecked() })
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
    range: Range<u8>
}

impl  AllRankIter {
    fn next_with(&mut self, f: impl FnOnce(&mut Range<u8>) -> Option<u8>) -> Option<Rank> {
        let rank = Rank::from_u8(f(&mut self.range)?);
        Some(unsafe{ rank.unwrap_unchecked() })
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
        write!(f, "{}", self .lower_letter())
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
fn test() { 
    extern crate std;
    for i in b'a'..=b'h' {
        let file = File::from_u8(i - b'a').unwrap();
        std::eprintln!("{i} {file}");
        assert_eq!(file, File::from_ascii_byte(i).unwrap())
    }
}
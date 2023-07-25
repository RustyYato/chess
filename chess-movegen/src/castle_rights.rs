use std::fmt::Write;

use chess_bitboard::{Color, Pos, Side};

const __: CastleRights = CastleRights::empty().not();
const WK: CastleRights = CastleRights::empty().with(Side::King, Color::White).not();
const WQ: CastleRights = CastleRights::empty().with(Side::Queen, Color::White).not();
const WB: CastleRights = CastleRights::empty()
    .with(Side::Queen, Color::White)
    .with(Side::King, Color::White)
    .not();

const BK: CastleRights = CastleRights::empty().with(Side::King, Color::Black).not();
const BQ: CastleRights = CastleRights::empty().with(Side::Queen, Color::Black).not();
const BB: CastleRights = CastleRights::empty()
    .with(Side::Queen, Color::Black)
    .with(Side::King, Color::Black)
    .not();

#[rustfmt::skip]
const CASTLE_RIGHTS_PER_SQ_INIT: [[CastleRights; 64]; 2] = [
    [
    WQ, __, __, __, WB, __, __, WK, 
    __, __, __, __, __, __, __, __, 
    __, __, __, __, __, __, __, __, 
    __, __, __, __, __, __, __, __, 
    __, __, __, __, __, __, __, __, 
    __, __, __, __, __, __, __, __, 
    __, __, __, __, __, __, __, __, 
    __, __, __, __, __, __, __, __, 
],
[
    __, __, __, __, __, __, __, __, 
    __, __, __, __, __, __, __, __, 
    __, __, __, __, __, __, __, __, 
    __, __, __, __, __, __, __, __, 
    __, __, __, __, __, __, __, __, 
    __, __, __, __, __, __, __, __, 
    __, __, __, __, __, __, __, __, 
    BQ, __, __, __, BB, __, __, BK, 
]];

static CASTLE_RIGHTS_PER_SQ: [[CastleRights; 64]; 2] = CASTLE_RIGHTS_PER_SQ_INIT;

const _: () = {
    assert!(CASTLE_RIGHTS_PER_SQ_INIT[Color::White as usize][Pos::A1 as usize].0 == WQ.0);
    assert!(CASTLE_RIGHTS_PER_SQ_INIT[Color::White as usize][Pos::H1 as usize].0 == WK.0);
    assert!(CASTLE_RIGHTS_PER_SQ_INIT[Color::White as usize][Pos::E1 as usize].0 == WB.0);

    assert!(CASTLE_RIGHTS_PER_SQ_INIT[Color::Black as usize][Pos::A8 as usize].0 == BQ.0);
    assert!(CASTLE_RIGHTS_PER_SQ_INIT[Color::Black as usize][Pos::H8 as usize].0 == BK.0);
    assert!(CASTLE_RIGHTS_PER_SQ_INIT[Color::Black as usize][Pos::E8 as usize].0 == BB.0);

    assert!(__.0 != WQ.0);
    assert!(__.0 != WK.0);
    assert!(__.0 != WB.0);

    assert!(__.0 != BQ.0);
    assert!(__.0 != BK.0);
    assert!(__.0 != BB.0);
};

#[repr(transparent)]
#[cfg_attr(feature = "abi_stable", derive(abi_stable::StableAbi))]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct CastleRights(u8);

impl core::fmt::Debug for CastleRights {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        static CASTLE_RIGHTS: [[char; 2]; 2] = [['K', 'Q'], ['k', 'q']];

        let rights = Color::all().flat_map(|color| {
            Side::all().filter_map(move |side| {
                if self.contains(side, color) {
                    Some(CASTLE_RIGHTS[color][side])
                } else {
                    None
                }
            })
        });

        for right in rights {
            f.write_char(right)?
        }

        if self.0 == 0 {
            f.write_str("-")?
        }

        Ok(())
    }
}

const fn offset(side: Side, color: Color) -> u32 {
    side as u32 + color as u32 * 2
}

impl CastleRights {
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }

    #[inline]
    pub const fn full() -> Self {
        Self::empty()
            .with(Side::King, Color::White)
            .with(Side::King, Color::Black)
            .with(Side::Queen, Color::White)
            .with(Side::Queen, Color::Black)
    }

    #[inline]
    pub const fn not(self) -> Self {
        Self(!self.0)
    }

    #[inline]
    pub const fn without(self, side: Side, color: Color) -> Self {
        Self(self.0 & !(1 << offset(side, color)))
    }

    #[inline]
    pub const fn with(self, side: Side, color: Color) -> Self {
        Self(self.0 | 1 << offset(side, color))
    }

    #[inline]
    pub fn remove(&mut self, side: Side, color: Color) {
        *self = self.without(side, color)
    }

    #[inline]
    pub fn add(&mut self, side: Side, color: Color) {
        *self = self.with(side, color)
    }

    #[inline]
    pub const fn contains(self, side: Side, color: Color) -> bool {
        self.0 & (1 << offset(side, color)) != 0
    }

    #[inline]
    pub const fn to_index(self) -> usize {
        let index = self.0 as usize;
        if index >= 16 {
            unsafe { core::hint::unreachable_unchecked() }
        }
        index
    }

    #[inline]
    pub const fn contains_color(self, color: Color) -> bool {
        self.0 & ((1 << offset(Side::King, color)) | (1 << offset(Side::Queen, color))) != 0
    }

    pub(crate) fn remove_for_sq(&mut self, turn: Color, end: Pos) {
        let rights = CASTLE_RIGHTS_PER_SQ[turn][end].0;
        self.0 &= rights;
    }
}

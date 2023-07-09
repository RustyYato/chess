use std::fmt::Write;

use chess_bitboard::{Color, Side};

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
    pub const fn contains_color(self, color: Color) -> bool {
        self.0 & ((1 << offset(Side::King, color)) | (1 << offset(Side::Queen, color))) != 0
    }
}

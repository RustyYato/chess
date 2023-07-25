use core::fmt::{Binary, Debug, Formatter, LowerHex, Result, UpperHex};

use crate::BitBoard;

impl Binary for BitBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:0>64b}", self.0)
    }
}

impl UpperHex for BitBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:0>16X}", self.0)
    }
}

impl LowerHex for BitBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:0>16x}", self.0)
    }
}

impl Debug for BitBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for rank in crate::Rank::all().rev() {
            for pos in rank {
                let c = match self.contains(pos) {
                    true => "# ",
                    false => ". ",
                };

                core::fmt::Display::fmt(c, f)?;
            }

            writeln!(f)?
        }

        Ok(())
    }
}

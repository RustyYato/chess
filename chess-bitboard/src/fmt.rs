use core::fmt::{Binary, Debug, Formatter, LowerHex, Result, UpperHex};

use crate::{BitBoard, Rank};

impl Binary for BitBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Binary::fmt(&self.0, f)
    }
}

impl UpperHex for BitBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        UpperHex::fmt(&self.0, f)
    }
}

impl LowerHex for BitBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        LowerHex::fmt(&self.0, f)
    }
}

impl Debug for BitBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for rank in Rank::all().rev() {
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

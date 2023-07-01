use core::fmt::{Binary, Debug, Formatter, LowerHex, Result, UpperHex};

use crate::BitBoard;

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
        for i in 0..64 {
            let bit = self.0 & (1 << i) != 0;

            let c = match bit {
                true => "# ",
                false => ". ",
            };

            core::fmt::Display::fmt(c, f)?;

            if i % 8 == 7 {
                writeln!(f)?
            }
        }

        Ok(())
    }
}

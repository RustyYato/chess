use std::ops::Index;

use chess_bitboard::{BitBoard, Color, Piece, Pos};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct RawBoard {
    colors: [BitBoard; 2],
    pieces: [BitBoard; 6],
}

impl core::fmt::Debug for RawBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            f.write_str("Color: White\n")?;
            self[Color::White].fmt(f)?;

            f.write_str("Color: Black\n")?;
            self[Color::Black].fmt(f)?;

            f.write_str("Piece: Pawn\n")?;
            self[Piece::Pawn].fmt(f)?;

            f.write_str("Piece: Knight\n")?;
            self[Piece::Knight].fmt(f)?;

            f.write_str("Piece: Bishop\n")?;
            self[Piece::Bishop].fmt(f)?;

            f.write_str("Piece: Rook\n")?;
            self[Piece::Rook].fmt(f)?;

            f.write_str("Piece: Queen\n")?;
            self[Piece::Queen].fmt(f)?;

            f.write_str("Piece: King\n")?;
            self[Piece::King].fmt(f)?;
        } else {
            static PIECES: [[char; 6]; 2] = [
                ['P', 'N', 'B', 'R', 'Q', 'K'],
                ['p', 'n', 'b', 'r', 'q', 'k'],
            ];
            write!(f, " ")?;
            for file in chess_bitboard::File::all() {
                write!(f, " {file:?}")?;
            }
            writeln!(f)?;
            for rank in chess_bitboard::Rank::all().rev() {
                write!(f, "{}", rank as u8 + 1)?;

                for file in chess_bitboard::File::all() {
                    let pos = chess_bitboard::Pos::new(file, rank);

                    match self.get(pos) {
                        Some((color, piece)) => {
                            let piece = PIECES[color][piece];
                            write!(f, " {piece}")?;
                        }
                        None => {
                            write!(f, " .")?;
                        }
                    }
                }

                writeln!(f)?;
            }
        }

        Ok(())
    }
}

impl core::fmt::Binary for RawBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_list();

        for i in self.colors {
            f.entry(&format_args!("{i:b}"));
        }

        for i in self.pieces {
            f.entry(&format_args!("{i:b}"));
        }

        f.finish()
    }
}

impl core::fmt::LowerHex for RawBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_list();

        for i in self.colors {
            f.entry(&format_args!("{i:x}"));
        }

        for i in self.pieces {
            f.entry(&format_args!("{i:x}"));
        }

        f.finish()
    }
}

impl core::fmt::UpperHex for RawBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_list();

        for i in self.colors {
            f.entry(&format_args!("{i:X}"));
        }

        for i in self.pieces {
            f.entry(&format_args!("{i:X}"));
        }

        f.finish()
    }
}

#[derive(Debug)]
pub struct PieceAlreadyExists;

impl RawBoard {
    pub const fn empty() -> Self {
        Self {
            colors: [BitBoard::empty(); 2],
            pieces: [BitBoard::empty(); 6],
        }
    }

    pub const fn standard() -> Self {
        Self {
            colors: [
                BitBoard::from_u64(0x0303030303030303),
                BitBoard::from_u64(0xc0c0c0c0c0c0c0c0),
            ],
            pieces: [
                BitBoard::from_u64(0x4242424242424242),
                BitBoard::from_u64(0x0081000000008100),
                BitBoard::from_u64(0x0000810000810000),
                BitBoard::from_u64(0x8100000000000081),
                BitBoard::from_u64(0x0000000081000000),
                BitBoard::from_u64(0x0000008100000000),
            ],
        }
    }

    #[inline]
    pub fn all(&self) -> BitBoard {
        self.colors[Color::White] | self.colors[Color::Black]
    }

    #[inline]
    pub fn set(&mut self, color: Color, piece: Piece, pos: Pos) -> Result<(), PieceAlreadyExists> {
        if self.all().contains(pos) {
            return Err(PieceAlreadyExists);
        }

        self.set_unchecked(color, piece, pos);

        Ok(())
    }

    #[inline]
    pub fn set_unchecked(&mut self, color: Color, piece: Piece, pos: Pos) {
        self.colors[color].set(pos);
        self.pieces[piece].set(pos);
    }

    #[inline]
    pub fn color_of(&self, pos: Pos) -> Option<Color> {
        if self.colors[Color::White].contains(pos) {
            Some(Color::White)
        } else if self.colors[Color::Black].contains(pos) {
            Some(Color::Black)
        } else {
            None
        }
    }

    #[inline]
    pub fn piece_of(&self, pos: Pos) -> Option<Piece> {
        self.color_of(pos)?;
        Some(unsafe { self.piece_of_unchecked(pos) })
        // Piece::all().find(|&piece| self.pieces[piece].contains(pos))
    }

    #[inline]
    pub unsafe fn piece_of_unchecked(&self, pos: Pos) -> Piece {
        let pieces = self[Piece::Pawn] | self[Piece::Knight] | self[Piece::Bishop];

        if pieces.contains(pos) {
            if self[Piece::Pawn].contains(pos) {
                Piece::Pawn
            } else if self[Piece::Knight].contains(pos) {
                Piece::Knight
            } else {
                Piece::Bishop
            }
        } else if self[Piece::Rook].contains(pos) {
            Piece::Rook
        } else if self[Piece::Queen].contains(pos) {
            Piece::Queen
        } else {
            debug_assert!(
                self[Piece::King].contains(pos),
                "Tried to get piece at {pos:?} where there is no piece, this is UB"
            );
            Piece::King
        }
    }

    #[inline]
    pub fn get(&self, pos: Pos) -> Option<(Color, Piece)> {
        let color = self.color_of(pos)?;
        let piece = unsafe { self.piece_of_unchecked(pos) };
        Some((color, piece))
    }

    #[inline]
    pub fn move_piece(&mut self, color: Color, piece: Piece, from: Pos, to: Pos) {
        self.remove(color, piece, from);
        self.set_unchecked(color, piece, to);
    }

    #[inline]
    pub fn remove(&mut self, color: Color, piece: Piece, pos: Pos) {
        self.colors[color] -= BitBoard::from_pos(pos);
        self.pieces[piece] -= BitBoard::from_pos(pos);
    }

    pub(crate) fn has_kings(&self) -> bool {
        let kings = self[Piece::King];
        let white = self[Color::White];
        let black = self[Color::Black];

        kings.count() == 2 && (kings & white).count() == 1 && (kings & black).count() == 1
    }
}

impl Index<Color> for RawBoard {
    type Output = BitBoard;

    #[inline]
    fn index(&self, index: Color) -> &Self::Output {
        &self.colors[index]
    }
}

impl Index<Piece> for RawBoard {
    type Output = BitBoard;

    #[inline]
    fn index(&self, index: Piece) -> &Self::Output {
        &self.pieces[index]
    }
}

impl RawBoard {
    pub(crate) fn xor(&mut self, color: Color, piece: Piece, diff: BitBoard) {
        debug_assert!(self.colors[color] & diff == self.pieces[piece] & diff);
        self.colors[color] ^= diff;
        self.pieces[piece] ^= diff;
    }
}

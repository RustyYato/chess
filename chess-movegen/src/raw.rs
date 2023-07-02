use std::ops::Index;

use chess_bitboard::{BitBoard, Color, Piece, Pos};

pub struct RawBoard {
    colors: [BitBoard; 2],
    pieces: [BitBoard; 6],
}

pub struct PieceAlreadyExists;

impl RawBoard {
    pub const fn empty() -> Self {
        Self {
            colors: [BitBoard::empty(); 2],
            pieces: [BitBoard::empty(); 6],
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
        Piece::all().find(|&piece| self.pieces[piece].contains(pos))
    }

    #[inline]
    pub unsafe fn piece_of_unchecked(&self, pos: Pos) -> Piece {
        unsafe { self.piece_of(pos).unwrap_unchecked() }
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

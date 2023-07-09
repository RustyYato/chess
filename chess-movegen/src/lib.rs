#![forbid(unsafe_op_in_unsafe_fn)]

mod castle_rights;
pub mod fen;
pub mod raw;

use std::str::FromStr;

use chess_bitboard::{BitBoard, Color, File, Piece, Pos, Side};
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Board {
    turn: Color,
    castle_rights: castle_rights::CastleRights,
    enpassant_target: Option<chess_bitboard::File>,
    half_move_clock: u16,
    full_move_clock: u16,
    pinned: BitBoard,
    checkers: BitBoard,
    raw: raw::RawBoard,
}

impl core::fmt::Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("turn: ")?;
        self.turn.fmt(f)?;
        f.write_str("\nhalf moves: ")?;
        self.half_move_clock.fmt(f)?;
        f.write_str("\nfull moves: ")?;
        self.full_move_clock.fmt(f)?;
        if let Some(ep) = self.enpassant_target {
            f.write_str("\nen-passant: ")?;
            ep.fmt(f)?;
        }
        f.write_str("\ncastle rights: ")?;
        self.castle_rights.fmt(f)?;
        f.write_str("\nboard:\n")?;
        self.raw.fmt(f)?;

        Ok(())
    }
}

impl FromStr for Board {
    type Err = fen::ParseFenError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fen::parse_fen(s.as_bytes())
    }
}

pub struct BoardBuilder {
    board: Board,
}

impl BoardBuilder {
    #[inline]
    pub fn turn(&mut self, turn: Color) -> &mut Self {
        self.board.turn = turn;
        self
    }

    #[inline]
    pub fn castle_rights(&mut self, rights: castle_rights::CastleRights) -> &mut Self {
        self.board.castle_rights = rights;
        self
    }

    #[inline]
    pub fn half_move_clock(&mut self, half_move_clock: u16) -> &mut Self {
        self.board.half_move_clock = half_move_clock;
        self
    }

    #[inline]
    pub fn full_move_clock(&mut self, full_move_clock: u16) -> &mut Self {
        self.board.full_move_clock = full_move_clock;
        self
    }

    #[inline]
    pub fn enpassant(&mut self, enpassant_target: impl Into<Option<File>>) -> &mut Self {
        self.board.enpassant_target = enpassant_target.into();
        self
    }

    #[inline]
    pub fn place(
        &mut self,
        pos: Pos,
        color: Color,
        piece: Piece,
    ) -> Result<&mut Self, raw::PieceAlreadyExists> {
        self.board.raw.set(color, piece, pos)?;
        Ok(self)
    }

    #[inline]
    pub fn remove(&mut self, pos: Pos) -> &mut Self {
        if let Some((color, piece)) = self.board.raw.get(pos) {
            self.board.raw.remove(color, piece, pos);
        }

        self
    }

    #[inline]
    pub fn build(&self) -> Result<Board, BoardValidationError> {
        let mut board = self.board;
        board.validate()?;
        board.update_pin_info();
        Ok(board)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BoardValidationError {
    MissingKings,
    InvalidCastleRights,
    InvalidEnpassant,
}

impl Board {
    pub const fn builder() -> BoardBuilder {
        BoardBuilder {
            board: Self {
                turn: Color::White,
                castle_rights: castle_rights::CastleRights::empty(),
                enpassant_target: None,
                half_move_clock: 0,
                full_move_clock: 0,
                pinned: BitBoard::empty(),
                checkers: BitBoard::empty(),
                raw: raw::RawBoard::empty(),
            },
        }
    }

    pub const fn standard() -> Self {
        Self {
            turn: Color::White,
            castle_rights: castle_rights::CastleRights::full(),
            enpassant_target: None,
            half_move_clock: 0,
            full_move_clock: 0,
            pinned: BitBoard::empty(),
            checkers: BitBoard::empty(),
            raw: raw::RawBoard::standard(),
        }
    }

    fn validate(&self) -> Result<(), BoardValidationError> {
        if !self.raw.has_kings() {
            return Err(BoardValidationError::MissingKings);
        }

        self.validate_en_passant()?;
        self.validate_castle_rights()?;

        Ok(())
    }

    fn validate_en_passant(&self) -> Result<(), BoardValidationError> {
        if let Some(ep) = self.enpassant_target {
            if self
                .raw
                .get(Pos::new(ep, self.turn.enpassant_capture_rank()))
                .is_some()
            {
                return Err(BoardValidationError::InvalidEnpassant);
            }

            if let Some((color, piece)) =
                self.raw.get(Pos::new(ep, self.turn.enpassant_pawn_rank()))
            {
                if color == self.turn {
                    return Err(BoardValidationError::InvalidEnpassant);
                }

                if piece != Piece::Pawn {
                    return Err(BoardValidationError::InvalidEnpassant);
                }
            } else {
                return Err(BoardValidationError::InvalidEnpassant);
            }
        }

        Ok(())
    }

    fn validate_castle_rights(&self) -> Result<(), BoardValidationError> {
        let cr = self.castle_rights;
        if cr.contains(Side::Queen, Color::White)
            && self.raw.get(Pos::A1) != Some((Color::White, Piece::Rook))
        {
            return Err(BoardValidationError::InvalidCastleRights);
        }

        if cr.contains(Side::King, Color::Black)
            && self.raw.get(Pos::H8) != Some((Color::Black, Piece::Rook))
        {
            return Err(BoardValidationError::InvalidCastleRights);
        }

        if cr.contains(Side::Queen, Color::Black)
            && self.raw.get(Pos::A8) != Some((Color::Black, Piece::Rook))
        {
            return Err(BoardValidationError::InvalidCastleRights);
        }

        if cr.contains_color(Color::White)
            && self.raw.get(Pos::E1) != Some((Color::White, Piece::King))
        {
            return Err(BoardValidationError::InvalidCastleRights);
        }

        if cr.contains_color(Color::Black)
            && self.raw.get(Pos::E8) != Some((Color::Black, Piece::King))
        {
            return Err(BoardValidationError::InvalidCastleRights);
        }

        Ok(())
    }

    pub fn raw(&self) -> raw::RawBoard {
        self.raw
    }

    #[inline]
    pub fn king_sq(&self, color: Color) -> chess_bitboard::Pos {
        let mut king_board = self.raw[color] & self.raw[Piece::King];
        unsafe { king_board.pop_unchecked() }
    }

    #[inline]
    pub fn turn(&self) -> Color {
        self.turn
    }

    pub fn update_pin_info(&mut self) {
        self.pinned = BitBoard::empty();
        self.checkers = BitBoard::empty();

        let king_pos = self.king_sq(self.turn);

        let bishop_rays = chess_lookup::bishop_rays(king_pos);
        let rook_rays = chess_lookup::rook_rays(king_pos);

        let queen_bb = self.raw[Piece::Queen];

        let bishop_pinners = (self.raw[Piece::Bishop] | queen_bb) & bishop_rays;
        let rook_pinners = (self.raw[Piece::Rook] | queen_bb) & rook_rays;

        let opp_bb = self.raw[!self.turn];

        let pinners = opp_bb & (bishop_pinners | rook_pinners);

        for pos in pinners {
            let between = chess_lookup::between(king_pos, pos);

            if between.none() {
                self.checkers.set(pos);
            } else if between.count() == 1 {
                self.pinned |= between;
            }
        }

        let knight_moves = chess_lookup::knight_moves(king_pos) & self.raw[Piece::Knight] & opp_bb;

        self.checkers |= knight_moves;

        let pawn_attacks =
            chess_lookup::pawn_attacks_moves(king_pos, self.turn) & self.raw[Piece::Pawn] & opp_bb;

        self.checkers |= pawn_attacks;
    }
}

#[cfg(test)]
mod tests {
    use crate::Board;

    #[test]
    fn test_standard_is_correct() {
        let board = Board::standard();

        assert_eq!(
            board,
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 0"
                .parse()
                .unwrap()
        );
    }
}

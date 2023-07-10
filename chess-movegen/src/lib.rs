#![forbid(unsafe_op_in_unsafe_fn)]

mod castle_rights;
pub mod fen;
mod iter;
pub mod raw;

use std::{
    fmt::{Debug, Write},
    hash::Hash,
    str::FromStr,
};

use chess_bitboard::{BitBoard, Color, File, Piece, Pos, PromotionPiece, Rank, Side};
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Board {
    zobrist: u64,
    turn: Color,
    castle_rights: castle_rights::CastleRights,
    enpassant_target: Option<chess_bitboard::File>,
    half_move_clock: u16,
    full_move_clock: u16,
    pinned: BitBoard,
    checkers: BitBoard,
    raw: raw::RawBoard,
}

impl Hash for Board {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.zobrist().hash(state);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChessMove {
    pub source: Pos,
    pub dest: Pos,
    pub promotion: Option<PromotionPiece>,
}

impl core::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        static PIECES: [[char; 6]; 2] = [
            ['P', 'N', 'B', 'R', 'Q', 'K'],
            ['p', 'n', 'b', 'r', 'q', 'k'],
        ];

        let mut missing: u32 = 0;

        for file in Rank::all().rev() {
            for pos in file {
                match self.raw.get(pos) {
                    Some((color, piece)) => {
                        if missing != 0 {
                            core::fmt::Display::fmt(&missing, f)?;
                            missing = 0;
                        }
                        f.write_char(PIECES[color][piece])?;
                    }
                    None => {
                        missing += 1;
                    }
                }
            }

            if missing != 0 {
                core::fmt::Display::fmt(&missing, f)?;
                missing = 0;
            }
            if file != Rank::_1 {
                f.write_str("/")?
            }
        }

        f.write_str(match self.turn {
            Color::White => " w ",
            Color::Black => " b ",
        })?;

        self.castle_rights.fmt(f)?;

        match self.enpassant_target {
            Some(file) => {
                let rank = self.turn.enpassant_pawn_rank();

                let file = (file as u8 + b'a') as char;
                f.write_str(" ")?;
                f.write_char(file)?;
                core::fmt::Display::fmt(&(rank as u8), f)?;
                f.write_str(" ")?;
            }
            None => f.write_str(" - ")?,
        }

        write!(f, "{} {}", self.half_move_clock, self.full_move_clock)
    }
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
        f.write_str("\nzobrist: ")?;
        self.zobrist.fmt(f)?;
        f.write_str("\nboard:\n")?;
        if f.alternate() {
            self.raw.fmt(f)?;
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

                    f.write_str(if self.pinned.contains(pos) {
                        "#"
                    } else if self.checkers.contains(pos) {
                        "*"
                    } else {
                        " "
                    })?;

                    match self.raw.get(pos) {
                        Some((color, piece)) => {
                            let piece = PIECES[color][piece];
                            write!(f, "{piece}")?;
                        }
                        None => {
                            write!(f, ".")?;
                        }
                    }
                }

                writeln!(f)?;
            }
        }

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
        self.board.zobrist ^= chess_lookup::zobrist(pos, piece, color);
        Ok(self)
    }

    #[inline]
    pub fn remove(&mut self, pos: Pos) -> &mut Self {
        if let Some((color, piece)) = self.board.raw.get(pos) {
            self.board.zobrist ^= chess_lookup::zobrist(pos, piece, color);
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
                zobrist: 0,
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
            zobrist: 2044085020143996643,
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
    pub fn half_move_clock(&self) -> u16 {
        self.half_move_clock
    }

    #[inline]
    pub fn full_move_clock(&self) -> u16 {
        self.full_move_clock
    }

    #[inline]
    pub fn zobrist(&self) -> u64 {
        self.zobrist
            ^ chess_lookup::turn_zobrist(self.turn)
            ^ self
                .enpassant_target
                .map_or(0, chess_lookup::en_passant_zobrist)
            ^ chess_lookup::castle_rights_zobrist(self.castle_rights.to_index())
    }

    #[inline]
    pub fn turn(&self) -> Color {
        self.turn
    }

    fn update_pin_info(&mut self) {
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

        let pieces = self.raw.all();

        for pos in pinners {
            let between = pieces & chess_lookup::between(king_pos, pos);

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

    #[inline]
    fn enpassant_pos(&self) -> Option<Pos> {
        self.enpassant_target
            .map(|file| Pos::new(file, self.turn.enpassant_capture_rank()))
    }

    /// # Safety
    ///
    /// * There must be a piece at mv.start
    /// * No king should be at mv.end
    /// * The color of mv.start must be self.turn
    /// * The color of mv.end must be !self.turn or an empty tile
    /// * mv.promotion must only be set if the moved piece is a pawn and it is moving to the promotion rank
    /// * mv must be a legal chess move
    #[inline]
    pub unsafe fn move_unchecked(&self, mv: ChessMove) -> Self {
        let mut board = *self;
        unsafe { self.move_unchecked_into(mv, &mut board) }
        board
    }

    /// # Safety
    ///
    /// * There must be a piece at mv.start
    /// * No king should be at mv.end
    /// * The color of mv.start must be self.turn
    /// * The color of mv.end must be !self.turn or an empty tile
    /// * mv.promotion must only be set if the moved piece is a pawn and it is moving to the promotion rank
    /// * mv must be a legal chess move
    #[inline]
    pub unsafe fn move_unchecked_mut(&mut self, mv: ChessMove) {
        let board = *self;
        unsafe { board.move_unchecked_into(mv, self) }
    }

    /// # Safety
    ///
    /// * There must be a piece at mv.start
    /// * No king should be at mv.end
    /// * The color of mv.start must be self.turn
    /// * The color of mv.end must be !self.turn or an empty tile
    /// * mv.promotion must only be set if the moved piece is a pawn and it is moving to the promotion rank
    /// * mv must be a legal chess move
    pub unsafe fn move_unchecked_into(&self, mv: ChessMove, output: &mut Self) {
        *output = *self;
        output.enpassant_target = None;
        output.checkers = BitBoard::empty();
        output.pinned = BitBoard::empty();
        output.turn = !self.turn;

        let source_bb = BitBoard::from(mv.source);
        let dest_bb = BitBoard::from(mv.dest);
        let mv_bb = source_bb ^ dest_bb;

        let piece = unsafe { self.raw.piece_of_unchecked(mv.source) };
        let captured = self.raw.piece_of(mv.dest);

        output.xor(self.turn, piece, mv_bb);
        if let Some(captured) = captured {
            output.xor(!self.turn, captured, dest_bb);
            output.half_move_clock = 0;
        } else {
            output.half_move_clock += 1;
        }
        output.full_move_clock += self.turn as u16;

        output.castle_rights.remove_for_sq(!self.turn, mv.dest);
        output.castle_rights.remove_for_sq(self.turn, mv.source);

        let opp_king = self.king_sq(!self.turn);
        let castles = piece == Piece::King && (mv_bb & chess_lookup::CASTLE_MOVES) == mv_bb;

        if piece == Piece::Knight {
            output.checkers ^= chess_lookup::knight_moves(opp_king) & dest_bb;
        } else if piece == Piece::Pawn {
            output.half_move_clock = 0;
            if let Some(promotion) = mv.promotion {
                debug_assert_eq!(mv.dest.rank(), chess_lookup::PROMOTION_RANK[self.turn]);

                // Bishop, Rook, and Queen checkers will be handled below
                if promotion == PromotionPiece::Knight {
                    output.checkers ^= chess_lookup::knight_moves(opp_king) & dest_bb;
                }

                output.xor(self.turn, Piece::Pawn, dest_bb);
                output.xor(self.turn, promotion.to_piece(), dest_bb);
            } else if mv_bb & chess_lookup::PAWN_DOUBLE_MOVE[self.turn] == mv_bb {
                output.enpassant_target = Some(mv.dest.file());
            } else if Some(mv.dest) == self.enpassant_pos() {
                let ep_file = mv.dest.file();

                // remove pawn by en-passant
                output.xor(
                    !self.turn,
                    Piece::Pawn,
                    BitBoard::from_pos(Pos::new(ep_file, self.turn.enpassant_pawn_rank())),
                );
            }

            if mv.promotion.is_none() {
                output.checkers ^= chess_lookup::pawn_attacks_moves(opp_king, !self.turn) & dest_bb;
            }
        } else if castles {
            let rook_mv = chess_lookup::BACKRANK_BB[self.turn]
                & match mv.dest.file().side() {
                    Side::King => chess_lookup::ROOK_CASTLE_KINGSIDE,
                    Side::Queen => chess_lookup::ROOK_CASTLE_QUEENSIDE,
                };

            output.xor(self.turn, Piece::Rook, rook_mv);
        }

        let pieces = output.raw[self.turn];
        let bishops = output.raw[Piece::Bishop] | output.raw[Piece::Queen];
        let rooks = output.raw[Piece::Rook] | output.raw[Piece::Queen];

        let attacking_bishops = bishops & pieces & chess_lookup::bishop_rays(opp_king);
        let attacking_rooks = rooks & pieces & chess_lookup::rook_rays(opp_king);

        let attackers = attacking_bishops | attacking_rooks;

        let opp_pieces = output.raw.all();

        for attacker in attackers {
            let between = opp_pieces & chess_lookup::between(opp_king, attacker);

            if between.none() {
                output.checkers.set(attacker);
            } else if between.count() == 1 {
                output.pinned ^= between;
            }
        }
    }

    #[inline]
    fn xor(&mut self, color: Color, piece: Piece, diff: BitBoard) {
        self.raw.xor(color, piece, diff);
        for pos in diff {
            self.zobrist ^= chess_lookup::zobrist(pos, piece, color);
        }
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

#![forbid(unsafe_op_in_unsafe_fn)]

mod castle_rights;
pub mod fen;
pub mod raw;

use std::str::FromStr;

use chess_bitboard::{BitBoard, Color, Piece};

pub struct Board {
    raw: raw::RawBoard,
    turn: Color,
    pinned: BitBoard,
    checkers: BitBoard,
    castle_rights: castle_rights::CastleRights,
    enpassant_target: Option<chess_bitboard::File>,
    half_move_clock: u16,
    full_move_clock: u16,
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

impl Board {
    pub fn standard() -> Self {
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 0"
            .parse()
            .unwrap()
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

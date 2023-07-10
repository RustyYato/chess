use chess_bitboard::{BitBoard, Color, Piece, Pos, Rank, Side};

use crate::Board;

use super::{LegalMovesAt, MoveList};

fn check_mask<const IS_IN_CHECK: bool>(board: &Board, king_sq: Pos) -> BitBoard {
    assert_eq!(board.checkers.count(), IS_IN_CHECK as u8);

    if IS_IN_CHECK {
        chess_lookup::between(king_sq, unsafe { { board.checkers }.pop_unchecked() })
            | board.checkers
    } else {
        !BitBoard::empty()
    }
}

pub(super) trait PieceType {
    const PIECE: Piece;
    const CAN_MOVE_IF_PINNED: bool = true;

    fn pseudo_legals(src: Pos, color: Color, combined: BitBoard, mask: BitBoard) -> BitBoard;

    fn legals<const IS_IN_CHECK: bool>(movelist: &mut MoveList, board: &Board, mask: BitBoard) {
        let all = board.raw.all();
        let my_pieces = board.raw[board.turn];
        let king_sq = board.king_sq(board.turn);

        let pieces = board.raw[Self::PIECE] & my_pieces;
        let check_mask = check_mask::<IS_IN_CHECK>(board, king_sq);

        for src in pieces & !board.pinned {
            let moves = Self::pseudo_legals(src, board.turn, all, mask);
            let moves = moves & check_mask;

            if moves.none() {
                continue;
            }

            unsafe {
                movelist.push_unchecked(LegalMovesAt {
                    src,
                    moves,
                    promotion: false,
                })
            }
        }

        if IS_IN_CHECK || !Self::CAN_MOVE_IF_PINNED {
            return;
        }

        for src in pieces & board.pinned {
            let moves = Self::pseudo_legals(src, board.turn, all, mask);
            let moves = moves & chess_lookup::line(src, king_sq);

            if moves.none() {
                continue;
            }

            unsafe {
                movelist.push_unchecked(LegalMovesAt {
                    src,
                    moves,
                    promotion: false,
                })
            }
        }
    }
}

pub(super) struct Pawn;
pub(super) struct Knight;
pub(super) struct Bishop;
pub(super) struct Rook;
pub(super) struct Queen;
pub(super) struct King;

impl PieceType for Pawn {
    const PIECE: Piece = Piece::Pawn;

    fn pseudo_legals(src: Pos, color: Color, combined: BitBoard, mask: BitBoard) -> BitBoard {
        chess_lookup::pawn_moves(src, color, combined) & mask
    }

    fn legals<const IS_IN_CHECK: bool>(movelist: &mut MoveList, board: &Board, mask: BitBoard) {
        let combined = board.raw.all();
        let my_pieces = board.raw[board.turn];
        let king_sq = board.king_sq(board.turn);

        let pieces = board.raw[Piece::Pawn] & my_pieces;

        let check_mask = check_mask::<IS_IN_CHECK>(board, king_sq);

        let seventh_rank = match board.turn {
            Color::White => Rank::_7,
            Color::Black => Rank::_2,
        };

        for src in pieces & !board.pinned {
            let moves = Self::pseudo_legals(src, board.turn, combined, mask);
            let moves = moves & check_mask;

            if moves.none() {
                continue;
            }

            unsafe {
                movelist.push_unchecked(LegalMovesAt {
                    src,
                    moves,
                    promotion: src.rank() == seventh_rank,
                });
            }
        }

        if !IS_IN_CHECK {
            for src in pieces & board.pinned {
                let moves = Self::pseudo_legals(src, board.turn, combined, mask);
                let moves = moves & chess_lookup::line(king_sq, src);

                if moves.none() {
                    continue;
                }

                unsafe {
                    movelist.push_unchecked(LegalMovesAt {
                        src,
                        moves,
                        promotion: src.rank() == seventh_rank,
                    });
                }
            }
        }

        if let Some(ep_file) = board.enpassant_target {
            let rank = board.turn.enpassant_pawn_rank();
            let files = chess_lookup::ADJACENT_FILES[ep_file];
            let dest_rank = board.turn.enpassant_capture_rank();
            let dest = BitBoard::from(Pos::new(ep_file, dest_rank));
            let capture_pawn = Pos::new(ep_file, rank);

            // if the opponent's pawn is checking the king (and the only piece checking the king)
            // or if the there is no check and the opponent's pawn doesn't block a check against our king
            // then we can capture it via en-passant with any unpinned pawn on the same rank and adjacent file as the
            // opponent's pawn
            if check_mask.contains(capture_pawn) && !board.pinned.contains(capture_pawn) {
                for src in BitBoard::from(rank) & files & pieces & !board.pinned {
                    unsafe {
                        movelist.push_unchecked(LegalMovesAt {
                            src,
                            moves: dest,
                            promotion: false,
                        });
                    }
                }
            }
        }
    }
}

impl Board {
    fn is_legal_king_position(&self, king_pos: Pos) -> bool {
        let bishop_rays = chess_lookup::bishop_rays(king_pos);
        let rook_rays = chess_lookup::rook_rays(king_pos);

        let queen_bb = self.raw[Piece::Queen];

        let bishop_pinners = (self.raw[Piece::Bishop] | queen_bb) & bishop_rays;
        let rook_pinners = (self.raw[Piece::Rook] | queen_bb) & rook_rays;

        let opp_bb = self.raw[!self.turn];

        let pinners = opp_bb & (bishop_pinners | rook_pinners);

        let actual_king_pos =
            BitBoard::from(self.king_sq(self.turn)) ^ BitBoard::from_pos(king_pos);

        let pieces = self.raw.all() ^ actual_king_pos;

        for pos in pinners {
            let between = pieces & chess_lookup::between(king_pos, pos);

            if between.none() {
                return false;
            }
        }

        let king_moves = chess_lookup::king_moves(king_pos) & self.raw[Piece::King] & opp_bb;
        let knight_moves = chess_lookup::knight_moves(king_pos) & self.raw[Piece::Knight] & opp_bb;
        let pawn_attacks =
            chess_lookup::pawn_attacks_moves(king_pos, self.turn) & self.raw[Piece::Pawn] & opp_bb;

        (king_moves | knight_moves | pawn_attacks).none()
    }
}

impl PieceType for King {
    const PIECE: Piece = Piece::King;

    fn pseudo_legals(src: Pos, _color: Color, _combined: BitBoard, mask: BitBoard) -> BitBoard {
        chess_lookup::king_moves(src) & mask
    }

    fn legals<const IS_IN_CHECK: bool>(movelist: &mut MoveList, board: &Board, mask: BitBoard) {
        let combined = board.raw.all();
        let king_sq = board.king_sq(board.turn);

        let mut moves = Self::pseudo_legals(king_sq, board.turn, combined, mask);
        let pseudo_legals = moves;

        for dest in pseudo_legals {
            if !board.is_legal_king_position(dest) {
                moves.clear(dest);
            }
        }

        if !IS_IN_CHECK {
            let data = [
                (
                    Side::King,
                    chess_lookup::KINGSIDE_CASTLE_FILES,
                    chess_lookup::KINGSIDE_CASTLE_SAFE_FILES,
                ),
                (
                    Side::Queen,
                    chess_lookup::QUEENSIDE_CASTLE_FILES,
                    chess_lookup::QUEENSIDE_CASTLE_SAFE_FILES,
                ),
            ];

            for (side, castle_files, castle_safe_files) in data {
                if !board.castle_rights.contains(side, board.turn) {
                    continue;
                }

                let backrank = chess_lookup::BACKRANK_BB[board.turn];
                let castle_tiles = castle_files & backrank;

                if board.castle_rights.contains(side, board.turn)
                    && (castle_tiles & combined).none()
                {
                    let no_check_sq = castle_safe_files & backrank;

                    debug_assert_eq!(no_check_sq.count(), 2);

                    if no_check_sq
                        .iter()
                        .all(|dest| board.is_legal_king_position(dest))
                    {
                        moves ^= castle_tiles & chess_lookup::CASTLE_MOVES
                    }
                }
            }
        }

        if moves.none() {
            return;
        }

        unsafe {
            movelist.push_unchecked(LegalMovesAt {
                src: king_sq,
                moves,
                promotion: false,
            })
        }
    }
}

impl PieceType for Knight {
    const PIECE: Piece = Piece::Knight;
    const CAN_MOVE_IF_PINNED: bool = false;

    fn pseudo_legals(src: Pos, _color: Color, _combined: BitBoard, mask: BitBoard) -> BitBoard {
        chess_lookup::knight_moves(src) & mask
    }
}

impl PieceType for Bishop {
    const PIECE: Piece = Piece::Bishop;

    fn pseudo_legals(src: Pos, _color: Color, combined: BitBoard, mask: BitBoard) -> BitBoard {
        chess_lookup::bishop_moves(src, combined) & mask
    }
}

impl PieceType for Rook {
    const PIECE: Piece = Piece::Rook;

    fn pseudo_legals(src: Pos, _color: Color, combined: BitBoard, mask: BitBoard) -> BitBoard {
        chess_lookup::rook_moves(src, combined) & mask
    }
}

impl PieceType for Queen {
    const PIECE: Piece = Piece::Queen;

    fn pseudo_legals(src: Pos, _color: Color, combined: BitBoard, mask: BitBoard) -> BitBoard {
        (chess_lookup::rook_moves(src, combined) | chess_lookup::bishop_moves(src, combined)) & mask
    }
}

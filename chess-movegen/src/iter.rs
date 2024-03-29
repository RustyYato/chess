use crate::{Board, ChessMove};
use chess_bitboard::{BitBoard, Color, Pos, PromotionPiece};

mod pieces;
use pieces::*;

const NO_CHECK: bool = false;
const IN_CHECK: bool = true;

static PROMOTION_PIECES: [PromotionPiece; 4] = [
    PromotionPiece::Queen,
    PromotionPiece::Rook,
    PromotionPiece::Bishop,
    PromotionPiece::Knight,
];

#[derive(Clone)]
pub struct MoveGen {
    moves: MoveList,
    promotions: core::slice::Iter<'static, PromotionPiece>,
    mask: BitBoard,
    index: usize,
}

type MoveList = arrayvec::ArrayVec<LegalMovesAt, 18>;

#[derive(Debug, Clone)]
struct LegalMovesAt {
    src: Pos,
    moves: BitBoard,
    promotion: bool,
}

impl Board {
    pub fn legals(&self) -> MoveGen {
        MoveGen {
            moves: self.collect_moves(!BitBoard::empty()),
            promotions: PROMOTION_PIECES.iter(),
            mask: !BitBoard::empty(),
            index: 0,
        }
    }

    pub fn legals_masked(&self, mask: BitBoard) -> MoveGen {
        MoveGen {
            moves: self.collect_moves(mask),
            promotions: PROMOTION_PIECES.iter(),
            mask,
            index: 0,
        }
    }

    pub fn king_legals(&self, turn: Color) -> MoveGen {
        MoveGen {
            moves: self.collect_king_moves(turn),
            promotions: PROMOTION_PIECES.iter(),
            mask: !BitBoard::empty(),
            index: 0,
        }
    }

    fn collect_moves(&self, mask: BitBoard) -> MoveList {
        let mut moves = MoveList::default();
        let movelist = &mut moves;

        let mask = !self.raw[self.turn] & mask;

        if self.checkers.none() {
            Pawn::legals::<NO_CHECK>(movelist, self, mask);
            Knight::legals::<NO_CHECK>(movelist, self, mask);
            Bishop::legals::<NO_CHECK>(movelist, self, mask);
            Rook::legals::<NO_CHECK>(movelist, self, mask);
            Queen::legals::<NO_CHECK>(movelist, self, mask);
            King::legals::<NO_CHECK>(movelist, self, mask);
        } else {
            if self.checkers.count() == 1 {
                Pawn::legals::<IN_CHECK>(movelist, self, mask);
                Knight::legals::<IN_CHECK>(movelist, self, mask);
                Bishop::legals::<IN_CHECK>(movelist, self, mask);
                Rook::legals::<IN_CHECK>(movelist, self, mask);
                Queen::legals::<IN_CHECK>(movelist, self, mask);
            }
            King::legals::<IN_CHECK>(movelist, self, mask);
        }

        moves
    }

    fn collect_king_moves(&self, turn: Color) -> MoveList {
        let mut moves = MoveList::default();
        let movelist = &mut moves;

        let mask = !self.raw[turn];

        if self.checkers.none() {
            King::king_legals::<NO_CHECK>(movelist, self, turn, mask);
        } else {
            King::king_legals::<IN_CHECK>(movelist, self, turn, mask);
        }

        moves
    }

    pub fn perft_test(&self, depth: usize) -> usize {
        let iterable = self.legals();

        let mut result: usize = 0;
        if depth == 1 {
            iterable.len()
        } else {
            let mut next_board = Board::standard();
            for m in iterable {
                unsafe { self.move_unchecked_into(m, &mut next_board) };
                result += next_board.perft_test(depth - 1);
            }
            result
        }
    }
}

impl MoveGen {
    pub fn is_empty(&self) -> bool {
        if let [legals, ..] = &self.moves[self.index..] {
            return (legals.moves & self.mask).none();
        }

        true
    }

    pub fn len(&self) -> usize {
        const NUM_PROMOTION_PIECES: usize = 4;

        let mut len = 0;

        for legals in &self.moves[self.index..] {
            if (legals.moves & self.mask).none() {
                break;
            }
            let count = (legals.moves & self.mask).count() as usize;
            len += if legals.promotion {
                count * NUM_PROMOTION_PIECES
            } else {
                count
            };
        }

        len
    }

    /// Never move to any position marked in the mask
    pub fn remove(&mut self, mask: BitBoard) {
        for legals in &mut self.moves {
            legals.moves -= mask;
        }
    }

    /// Never, ever, iterate this move
    pub fn remove_move(&mut self, chess_move: ChessMove) -> bool {
        for x in 0..self.moves.len() {
            if self.moves[x].src == chess_move.source {
                self.moves[x].moves -= chess_move.dest;
                return true;
            }
        }
        false
    }

    pub fn set_mask(&mut self, mask: BitBoard) {
        self.mask = mask;
        self.index = 0;

        let moves = &mut self.moves[..];

        let mut i = moves.as_mut_ptr();
        let mut j = i;
        let end = unsafe { i.add(moves.len()) };

        unsafe {
            while i < end {
                if ((*i).moves & mask).any() {
                    if i != j {
                        #[allow(clippy::swap_ptr_to_ref)]
                        core::mem::swap(&mut *i, &mut *j)
                    }

                    i = i.add(1);
                    j = j.add(1);
                } else {
                    i = i.add(1);
                }
            }
        }
    }
}

impl ExactSizeIterator for MoveGen {}
impl Iterator for MoveGen {
    type Item = ChessMove;

    fn next(&mut self) -> Option<Self::Item> {
        let legals = &mut self.moves[..];
        if self.index >= legals.len() {
            return None;
        }

        let legal = &mut legals[self.index];

        if (legal.moves & self.mask).none() {
            return None;
        }

        if legal.promotion {
            let &promotion = self.promotions.next().unwrap();

            let mut moves = legal.moves & self.mask;
            let dest = unsafe { moves.pop_unchecked() };

            let result = ChessMove {
                source: legal.src,
                dest,
                piece: Some(promotion),
            };

            if self.promotions.len() == 0 {
                self.promotions = PROMOTION_PIECES.iter();

                legal.moves.clear(dest);

                if (moves & self.mask).none() {
                    self.index += 1;
                }
            }

            Some(result)
        } else {
            let mut possible_moves = legal.moves & self.mask;
            let dest = unsafe { possible_moves.pop_unchecked() };
            legal.moves.clear(dest);

            let result = ChessMove {
                source: legal.src,
                dest,
                piece: None,
            };

            if possible_moves.none() {
                self.index += 1;
            }

            Some(result)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }

    fn count(self) -> usize {
        self.len()
    }
}

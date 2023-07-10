use crate::{Board, ChessMove};
use chess_bitboard::{BitBoard, Pos, PromotionPiece};

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

pub struct MoveGen {
    moves: MoveList,
    promotions: core::slice::Iter<'static, PromotionPiece>,
    mask: BitBoard,
    index: usize,
}

type MoveList = arrayvec::ArrayVec<LegalMovesAt, 18>;

#[derive(Debug)]
struct LegalMovesAt {
    src: Pos,
    moves: BitBoard,
    promotion: bool,
}

impl Board {
    pub fn legals(&self) -> MoveGen {
        MoveGen {
            moves: self.collect_moves(),
            promotions: PROMOTION_PIECES.iter(),
            mask: !BitBoard::empty(),
            index: 0,
        }
    }

    fn collect_moves(&self) -> MoveList {
        let mut moves = MoveList::default();
        let movelist = &mut moves;

        let mask = !self.raw[self.turn];

        if self.checkers.none() {
            Pawn::legals::<NO_CHECK>(movelist, self, mask);
            Knight::legals::<NO_CHECK>(movelist, self, mask);
            Bishop::legals::<NO_CHECK>(movelist, self, mask);
            Rook::legals::<NO_CHECK>(movelist, self, mask);
            Queen::legals::<NO_CHECK>(movelist, self, mask);
            King::legals::<NO_CHECK>(movelist, self, mask);
        } else if self.checkers.count() == 1 {
            Pawn::legals::<IN_CHECK>(movelist, self, mask);
            Knight::legals::<IN_CHECK>(movelist, self, mask);
            Bishop::legals::<IN_CHECK>(movelist, self, mask);
            Rook::legals::<IN_CHECK>(movelist, self, mask);
            Queen::legals::<IN_CHECK>(movelist, self, mask);
            King::legals::<IN_CHECK>(movelist, self, mask);
        } else {
            King::legals::<IN_CHECK>(movelist, self, mask);
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

        self.index = unsafe { j.offset_from(moves.as_ptr()) } as usize
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

            let mut moves = legal.moves;

            let result = ChessMove {
                source: legal.src,
                dest: unsafe { moves.pop_unchecked() },
                promotion: Some(promotion),
            };

            if self.promotions.len() == 0 {
                self.promotions = PROMOTION_PIECES.iter();

                legal.moves = moves;

                if (moves & self.mask).none() {
                    self.index += 1;
                }
            }

            Some(result)
        } else {
            let result = ChessMove {
                source: legal.src,
                dest: unsafe { legal.moves.pop_unchecked() },
                promotion: None,
            };

            if (legal.moves & self.mask).none() {
                self.index += 1;
            }

            Some(result)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        const NUM_PROMOTION_PIECES: usize = 4;

        let mut result = 0;
        for legals in &self.moves[self.index..] {
            if (legals.moves & self.mask).none() {
                break;
            }
            let count = (legals.moves & self.mask).count() as usize;
            result += if legals.promotion {
                count * NUM_PROMOTION_PIECES
            } else {
                count
            };
        }
        (result, Some(result))
    }

    fn count(self) -> usize {
        self.len()
    }
}

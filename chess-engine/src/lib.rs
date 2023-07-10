mod score;

use std::{
    collections::BTreeMap,
    time::{Duration, Instant},
};

use chess_bitboard::{Color, Piece};
use chess_movegen::{Board, ChessMove};
use score::Score;

#[derive(Default)]
pub struct Engine {
    moves_evaluated: u64,
    cutoffs: BTreeMap<u16, u64>,
}

pub struct DurationTimeout {
    deadline: Instant,
}

impl DurationTimeout {
    pub fn new(duration: Duration) -> Self {
        Self {
            deadline: std::time::Instant::now() + duration,
        }
    }
}

impl Timeout for DurationTimeout {
    fn is_complete(&self) -> bool {
        Instant::now() >= self.deadline
    }
}

pub trait Timeout {
    #[must_use]
    fn is_complete(&self) -> bool;
}

impl<T: ?Sized + Timeout> Timeout for &T {
    #[inline]
    fn is_complete(&self) -> bool {
        T::is_complete(self)
    }
}

trait Policy {
    type Flip: Policy<Flip = Self>;
    const COLOR: Color;
    const WORST_SCORE: Score;
    const BEST_SCORE: Score;

    fn is_better(score: Score, new: Score) -> bool;

    fn update_cutoff(alpha: &mut Score, beta: &mut Score, score: Score) -> bool;
}

struct White;
struct Black;

impl Policy for White {
    type Flip = Black;
    const COLOR: Color = Color::White;
    const WORST_SCORE: Score = Score::Min;
    const BEST_SCORE: Score = Score::Max;

    #[inline]
    fn is_better(score: Score, new: Score) -> bool {
        score < new
    }

    #[inline]
    fn update_cutoff(alpha: &mut Score, beta: &mut Score, score: Score) -> bool {
        *alpha = score;
        score >= *beta
    }
}

impl Policy for Black {
    type Flip = White;
    const COLOR: Color = Color::Black;
    const WORST_SCORE: Score = Score::Max;
    const BEST_SCORE: Score = Score::Min;

    #[inline]
    fn is_better(score: Score, new: Score) -> bool {
        score > new
    }

    #[inline]
    fn update_cutoff(alpha: &mut Score, beta: &mut Score, score: Score) -> bool {
        *beta = score;
        score <= *alpha
    }
}

struct SearchState<'a> {
    current: &'a Board,
    next_board: Board,
    best_mv: Option<ChessMove>,
    score: Score,
    depth: u16,
}

impl Engine {
    pub fn search(
        &mut self,
        board: &Board,
        timeout: &(impl Timeout + ?Sized),
    ) -> (Option<ChessMove>, Score) {
        match board.turn() {
            chess_bitboard::Color::White => self.search_::<White, _>(board, timeout),
            chess_bitboard::Color::Black => self.search_::<Black, _>(board, timeout),
        }
    }

    fn search_<P: Policy, T: Timeout + Copy>(
        &mut self,
        board: &Board,
        timeout: T,
    ) -> (Option<ChessMove>, Score) {
        let moves = board.legals();

        self.moves_evaluated = 0;
        self.cutoffs.clear();

        let mut state = SearchState {
            current: board,
            next_board: Board::standard(),
            score: P::WORST_SCORE,
            best_mv: None,
            depth: 1,
        };

        loop {
            let mut moves = moves.clone();

            if let Some(best_mv) = state.best_mv {
                moves.remove_move(best_mv);
                self.search_root_move::<P, _>(best_mv, &mut state, timeout)
            }

            let opp = board[!P::COLOR];

            for piece in [
                Piece::Queen,
                Piece::Rook,
                Piece::Bishop,
                Piece::Knight,
                Piece::Pawn,
            ] {
                let opp_pieces = board[piece] & opp;

                if opp_pieces.none() {
                    continue;
                }

                moves.set_mask(opp_pieces);

                for mv in &mut moves {
                    self.search_root_move::<P, _>(mv, &mut state, timeout)
                }
            }

            moves.set_mask(!chess_bitboard::BitBoard::empty());

            for mv in moves {
                self.search_root_move::<P, _>(mv, &mut state, timeout)
            }

            if timeout.is_complete() {
                break;
            }

            state.depth += 1;
        }

        (state.best_mv, state.score)
    }

    fn search_root_move<P: Policy, T: Timeout + Copy>(
        &mut self,
        mv: ChessMove,
        state: &mut SearchState<'_>,
        timeout: T,
    ) {
        if timeout.is_complete() {
            return;
        }

        unsafe { state.current.move_unchecked_into(mv, &mut state.next_board) }
        let new: Score = self.search_to::<P, T>(
            &state.next_board,
            state.depth,
            0,
            Score::Min,
            Score::Max,
            timeout,
        );

        if P::is_better(state.score, new) {
            state.score = new;
            state.best_mv = Some(mv);
        }
    }

    fn search_to<P: Policy, T: Timeout + Copy>(
        &mut self,
        board: &Board,
        depth: u16,
        current_depth: u16,
        mut alpha: Score,
        mut beta: Score,
        timeout: T,
    ) -> Score {
        let moves = board.legals();

        if moves.len() == 0 {
            return if board.in_check() {
                // if white has no moves, and is in check
                // then black mated them and vice versa
                match P::COLOR {
                    Color::White => Score::BlackMateIn(current_depth),
                    Color::Black => Score::WhiteMateIn(current_depth),
                }
            } else {
                Score::Raw(0)
            };
        }

        if depth == 0 {
            return self.eval(board);
        }

        let mut score = P::WORST_SCORE;
        let mut next_board = Board::standard();

        for mv in moves {
            if timeout.is_complete() {
                break;
            }

            unsafe { board.move_unchecked_into(mv, &mut next_board) }
            let new = self.search_to::<P::Flip, T>(
                &next_board,
                depth - 1,
                current_depth + 1,
                alpha,
                beta,
                timeout,
            );

            if !P::is_better(score, new) {
                continue;
            }

            if P::update_cutoff(&mut alpha, &mut beta, new) {
                *self.cutoffs.entry(depth).or_default() += 1;
                break;
            }

            score = new;
        }

        score
    }

    fn eval(&mut self, board: &Board) -> Score {
        self.moves_evaluated += 1;

        if board.half_move_clock() >= 100 {
            return Score::Raw(0);
        }

        let white_score = self.score_pieces(board, Color::White);
        let black_score = self.score_pieces(board, Color::Black);

        Score::Raw(white_score - black_score)
    }

    fn score_pieces(&mut self, board: &Board, color: Color) -> i16 {
        let my_pieces = board[color];

        let my_queen_score = (my_pieces & board[Piece::Queen]).count() as i16 * 800;
        let my_rook_score = (my_pieces & board[Piece::Rook]).count() as i16 * 500;
        let my_bishop_score = (my_pieces & board[Piece::Bishop]).count() as i16 * 330;
        let my_knight_score = (my_pieces & board[Piece::Knight]).count() as i16 * 300;
        let my_pawn_score = (my_pieces & board[Piece::Pawn]).count() as i16 * 100;

        my_queen_score + my_rook_score + my_bishop_score + my_knight_score + my_pawn_score
    }
}

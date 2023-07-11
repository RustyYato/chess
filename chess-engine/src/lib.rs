mod score;

use std::{
    collections::BTreeMap,
    time::{Duration, Instant},
};

use chess_bitboard::{Color, Piece};
use chess_movegen::{Board, ChessMove};
use owo_colors::OwoColorize as _;
use score::Score;

#[derive(Default)]
pub struct Engine {
    pub moves_evaluated: u64,
    pub cutoffs: BTreeMap<u16, u64>,
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
        if *alpha <= score {
            *alpha = score;
            score > *beta
        } else {
            true
        }
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
        if *beta >= score {
            *beta = score;
            score < *alpha
        } else {
            true
        }
    }
}

struct SearchState<'a> {
    board: &'a Board,
    best_mv: Option<ChessMove>,
    score: Score,
    alpha: Score,
    beta: Score,
    depth: u16,
}

#[derive(Clone, Copy)]
struct BoardList<'a> {
    prev: Option<&'a BoardList<'a>>,
    board: &'a Board,
    count: u8,
}

impl<'a> BoardList<'a> {
    pub fn new(board: &'a Board) -> Self {
        Self {
            prev: None,
            board,
            count: 1,
        }
    }

    pub fn add(&'a self, board: &'a Board) -> Self {
        Self {
            prev: Some(self),
            board,
            count: self.count(board) + 1,
        }
    }

    pub fn count(&self, board: &Board) -> u8 {
        if self.board == board {
            self.count
        } else {
            self.prev.map_or(0, |list| list.count(board))
        }
    }
}

impl Engine {
    pub fn search(
        &mut self,
        board: &Board,
        timeout: &(impl Timeout + ?Sized),
    ) -> (Option<ChessMove>, Score) {
        if self.insuffient_material(board) {
            return (None, Score::Raw(0));
        }

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
            board,
            score: P::WORST_SCORE,
            alpha: Score::Min,
            beta: Score::Max,
            best_mv: None,
            depth: 1,
        };

        loop {
            // eprintln!("depth = {}", state.depth);
            let mut moves = moves.clone();

            state.score = P::WORST_SCORE;
            state.alpha = Score::Min;
            state.beta = Score::Max;

            if let Some(best_mv) = state.best_mv {
                // eprintln!("best {best_mv:?}");
                moves.remove_move(best_mv);
                // extend the search for the best move by one
                // so we can eliminate it if it goes wrong in the future
                state.depth += 1;
                self.search_root_move::<P, _>(best_mv, &mut state, timeout);
                state.depth -= 1
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

            // eprintln!("{:?}\t{:?}", state.score, self.cutoffs.last_entry());

            if timeout.is_complete() {
                break;
            }

            tracing::trace!(depth = state.depth, "finish depth");
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
        tracing::trace!(chess_move=?mv, "{}", "consider".bright_yellow());
        if timeout.is_complete() {
            return;
        }

        let new = self.search_to::<P, T>(
            state.board,
            mv,
            state.depth,
            0,
            state.alpha,
            state.beta,
            BoardList::new(state.board),
            timeout,
        );

        if P::is_better(state.score, new) {
            state.score = new;
            state.best_mv = Some(mv);

            P::update_cutoff(&mut state.alpha, &mut state.beta, new);
        }
    }

    fn search_captures<P: Policy, T: Timeout + Copy>(
        &mut self,
        board: &Board,
        mut alpha: Score,
        mut beta: Score,
        timeout: T,
    ) -> Score {
        let moves = board.legals_masked(board[!board.turn()]);

        let mut score = P::WORST_SCORE;
        let mut next_board = Board::standard();

        if moves.len() == 0 {
            return self.eval(board);
        }

        for mv in moves {
            if timeout.is_complete() {
                break;
            }

            unsafe { board.move_unchecked_into(mv, &mut next_board) }
            let new = self.search_captures::<P::Flip, T>(&next_board, alpha, beta, timeout);

            if !P::is_better(score, new) {
                continue;
            }

            score = new;

            if P::update_cutoff(&mut alpha, &mut beta, new) {
                break;
            }
        }

        score
    }

    fn search_to<P: Policy, T: Timeout + Copy>(
        &mut self,
        prev_board: &Board,
        mv: ChessMove,
        depth: u16,
        current_depth: u16,
        mut alpha: Score,
        mut beta: Score,
        list: BoardList<'_>,
        timeout: T,
    ) -> Score {
        let was_capture = prev_board.raw().get(mv.dest).is_some();
        let board = unsafe { prev_board.move_unchecked(mv) };
        let moves = board.legals();

        let list = if was_capture
            || prev_board
                .raw()
                .get(mv.source)
                .is_some_and(|(_, piece)| piece == Piece::Pawn)
        {
            BoardList::new(&board)
        } else {
            list.add(&board)
        };

        if list.count == 3 || (was_capture && self.insuffient_material(&board)) {
            return Score::Raw(0);
        }

        if moves.len() == 0 {
            return if board.in_check() {
                // if white has no moves, and is in check
                // then black mated them and vice versa
                match P::COLOR {
                    Color::White => Score::WhiteMateIn(current_depth),
                    Color::Black => Score::BlackMateIn(current_depth),
                }
            } else {
                Score::Raw(0)
            };
        }

        if depth == 0 {
            return if was_capture {
                self.search_captures::<P, T>(&board, alpha, beta, timeout)
            } else {
                self.eval(&board)
            };
        }

        let mut score = if was_capture {
            self.search_captures::<P, T>(&board, alpha, beta, timeout)
        } else {
            P::WORST_SCORE
        };

        tracing::trace!(current_depth, color=?P::COLOR, depth, ?alpha, ?beta, "{}", "start".yellow());

        for mv in moves {
            if timeout.is_complete() {
                break;
            }

            let new = self.search_to::<P::Flip, T>(
                &board,
                mv,
                depth - 1,
                current_depth + 1,
                alpha,
                beta,
                list,
                timeout,
            );

            if !P::is_better(score, new) {
                tracing::trace!(current_depth, color=?P::COLOR, depth, ?alpha, ?beta, ?score, ?new, "{}", "not better".bright_red());
                continue;
            }

            score = new;

            if P::update_cutoff(&mut alpha, &mut beta, new) {
                tracing::trace!(current_depth, color=?P::COLOR, depth, ?alpha, ?beta, ?score, ?new, "{}", "cutoff".bright_green());
                *self.cutoffs.entry(depth).or_default() += 1;
                break;
            } else {
                tracing::trace!(current_depth, color=?P::COLOR, depth, ?alpha, ?beta, ?score, ?new, "{}", "better".bright_cyan());
            }
        }

        tracing::trace!(current_depth, color=?P::COLOR, depth, ?alpha, ?beta, ?score, "{}", "eval".bright_blue());

        score
    }

    fn insuffient_material(&self, board: &Board) -> bool {
        let board = board.raw();

        let queens_or_rooks_or_pawns =
            board[Piece::Queen] | board[Piece::Rook] | board[Piece::Pawn];

        if queens_or_rooks_or_pawns.any() {
            return false;
        }

        let bishops = board[Piece::Bishop].count();
        let knights = board[Piece::Knight].count();

        knights <= 1 && bishops == 0 || knights == 0 && bishops <= 1
    }

    fn eval(&mut self, board: &Board) -> Score {
        self.moves_evaluated += 1;

        if board.half_move_clock() >= 100 {
            return Score::Raw(0);
        }

        let mut white_score = self.score_pieces(board, Color::White);
        let mut black_score = self.score_pieces(board, Color::Black);

        let piece_score = white_score - black_score;

        match piece_score.cmp(&0) {
            std::cmp::Ordering::Less => {
                if black_score < 800 + 500 * 3 {
                    // if we are in the endgame
                    let white_king = board.king_sq(Color::White);
                    let black_king = board.king_sq(Color::Black);

                    let dist = chess_lookup::distance(white_king, black_king);
                    // minimize the distance to the
                    black_score -= (dist as i32) * 100;
                    // penalized for staying close to the edge
                    white_score -= DIST_FROM_CENTER[white_king] as i32 * 100;
                    // black_score -= DIST_FROM_CENTER[black_king] as i32 * 30;
                }
            }
            std::cmp::Ordering::Equal => (),
            std::cmp::Ordering::Greater => {
                if white_score < 800 + 500 * 3 {
                    // if we are in the endgame
                    let white_king = board.king_sq(Color::White);
                    let black_king = board.king_sq(Color::Black);

                    let dist = chess_lookup::distance(white_king, black_king);
                    // minimize the distance to the
                    white_score -= (dist as i32) * 100;
                    // penalized for staying close to the edge
                    // white_score -= DIST_FROM_CENTER[white_king] as i32 * 30;
                    black_score -= DIST_FROM_CENTER[black_king] as i32 * 100;
                }
            }
        }

        let piece_score = white_score - black_score;

        Score::Raw(piece_score.try_into().unwrap())
    }

    fn score_pieces(&mut self, board: &Board, color: Color) -> i32 {
        let my_pieces = board[color];

        let my_queen_score = (my_pieces & board[Piece::Queen]).count() as i32 * 800;
        let my_rook_score = (my_pieces & board[Piece::Rook]).count() as i32 * 500;
        let my_bishop_score = (my_pieces & board[Piece::Bishop]).count() as i32 * 330;
        let my_knight_score = (my_pieces & board[Piece::Knight]).count() as i32 * 300;
        let my_pawn_score = (my_pieces & board[Piece::Pawn]).count() as i32 * 100;

        my_queen_score + my_rook_score + my_bishop_score + my_knight_score + my_pawn_score
    }
}

#[rustfmt::skip]
static DIST_FROM_CENTER: [u8; 64] = [
    3, 3, 3, 3, 3, 3, 3, 3,
    3, 2, 2, 2, 2, 2, 2, 3,
    3, 2, 1, 1, 1, 1, 2, 3,
    3, 2, 1, 0, 0, 1, 2, 3,
    3, 2, 1, 0, 0, 1, 2, 3,
    3, 2, 1, 1, 1, 1, 2, 3,
    3, 2, 2, 2, 2, 2, 2, 3,
    3, 3, 3, 3, 3, 3, 3, 3,
];

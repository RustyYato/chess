mod score;

use std::time::{Duration, Instant};

use chess_bitboard::{Color, Piece};
use chess_movegen::{Board, ChessMove};
use colorz::Colorize as _;
pub use score::Score;

#[derive(Default)]
pub struct Engine {
    pub moves_evaluated: u64,
    pub max_depth: u16,
}

#[derive(Debug, Clone, Copy)]
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

impl<T: Timeout + Copy> TimeoutRef for T {}
pub trait TimeoutRef: Timeout + Copy {}

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

    const IS_BETA_CUTOFF: bool = false;

    fn is_better(score: Score, new: Score) -> bool;

    fn update_cutoff(alpha: &mut Score, beta: &mut Score, score: Score);
}

struct White;
struct Black;

impl Policy for White {
    type Flip = Black;
    const COLOR: Color = Color::White;

    const WORST_SCORE: Score = Score::Min;
    const BEST_SCORE: Score = Score::Max;

    const IS_BETA_CUTOFF: bool = true;

    fn is_better(score: Score, new: Score) -> bool {
        score < new
    }

    fn update_cutoff(alpha: &mut Score, _beta: &mut Score, score: Score) {
        *alpha = score.max(*alpha)
    }
}

impl Policy for Black {
    type Flip = White;
    const COLOR: Color = Color::Black;

    const WORST_SCORE: Score = Score::Max;
    const BEST_SCORE: Score = Score::Min;

    const IS_BETA_CUTOFF: bool = false;

    fn is_better(score: Score, new: Score) -> bool {
        score > new
    }

    fn update_cutoff(_alpha: &mut Score, beta: &mut Score, score: Score) {
        *beta = score.min(*beta)
    }
}

struct AlphaBetaArgs<'a, T> {
    old_board: &'a Board,
    mv: ChessMove,
    timeout: T,
    remaining_depth: u16,
    current_depth: u16,
    alpha: Score,
    beta: Score,
}

impl Engine {
    pub fn search(
        &mut self,
        board: &Board,
        timeout: impl TimeoutRef,
    ) -> (Option<ChessMove>, Score) {
        match board.turn() {
            Color::White => self.search_with::<White>(board, timeout),
            Color::Black => self.search_with::<Black>(board, timeout),
        }
    }

    fn search_with<P: Policy>(
        &mut self,
        board: &Board,
        timeout: impl TimeoutRef,
    ) -> (Option<ChessMove>, Score) {
        assert_eq!(P::COLOR, board.turn());

        let mut best_score = P::WORST_SCORE;
        let mut best_mv = None;

        let mut depth = 0;

        loop {
            if depth > 1 {
                break;
            }
            tracing::debug!(color = ?P::COLOR, depth, board=%board, "start depth");
            let mut score = P::WORST_SCORE;
            let mut best_mv_at = None;

            for mv in board.legals() {
                tracing::debug!(color = ?P::COLOR, depth, "move"=%mv, board=%board, "consider move");
                let new = self.alphabeta::<P>(AlphaBetaArgs {
                    old_board: board,
                    mv,
                    timeout,
                    remaining_depth: depth,
                    current_depth: 1,
                    alpha: Score::Min,
                    beta: Score::Max,
                });

                if timeout.is_complete() {
                    break;
                }

                if P::is_better(score, new) {
                    score = new;
                    best_mv_at = Some(mv);
                    tracing::debug!(color = ?P::COLOR, depth, "move"=%mv, board=%board, ?score, "{}", "better".bright_green());
                }
            }

            if timeout.is_complete() {
                break;
            }

            best_mv = best_mv_at;
            best_score = score;
            depth += 1;
        }

        (best_mv, best_score)
    }

    fn alphabeta<P: Policy>(&mut self, mut args: AlphaBetaArgs<'_, impl TimeoutRef>) -> Score {
        tracing::trace!(
            current_depth=args.current_depth,
            depth=args.remaining_depth,
            color=?P::COLOR,
            alpha=?args.alpha,
            beta=?args.beta,
            "move"=%args.mv,
            board=%args.old_board,
            "start alphabeta"
        );
        let board = unsafe { args.old_board.move_unchecked(args.mv) };
        let was_capture = args.old_board[!P::COLOR] != board[!P::COLOR];

        if was_capture && self.insuffient_material(&board) {
            tracing::trace!(
                current_depth=args.current_depth,
                depth=args.remaining_depth,
                color=?P::COLOR,
                alpha=?args.alpha,
                beta=?args.beta,
                "move"=%args.mv,
                board=%args.old_board,
                "{}", "tie (material)".bright_blue()
            );
            return Score::Raw(0);
        }

        let moves = board.legals();

        if moves.is_empty() {
            if board.in_check() {
                tracing::trace!(
                    current_depth=args.current_depth,
                    depth=args.remaining_depth,
                    color=?P::COLOR,
                    alpha=?args.alpha,
                    beta=?args.beta,
                    "move"=%args.mv,
                    board=%args.old_board,
                    "{}", "mate".bright_blue()
                );
                return match P::COLOR {
                    Color::White => Score::WhiteMateIn(args.current_depth),
                    Color::Black => Score::BlackMateIn(args.current_depth),
                };
            } else {
                tracing::trace!(
                    current_depth=args.current_depth,
                    depth=args.remaining_depth,
                    color=?P::COLOR,
                    alpha=?args.alpha,
                    beta=?args.beta,
                    "move"=%args.mv,
                    board=%args.old_board,
                    "{}", "tie (no moves)".bright_blue()
                );
                return Score::Raw(0);
            }
        }

        if board.half_move_clock() >= 100 {
            tracing::trace!(
                current_depth=args.current_depth,
                depth=args.remaining_depth,
                color=?P::COLOR,
                alpha=?args.alpha,
                beta=?args.beta,
                "move"=%args.mv,
                board=%args.old_board,
                "{}", "tie (clock)".bright_blue()
            );
            return Score::Raw(0);
        }

        if args.remaining_depth == 0 {
            let score = self.eval(&board, args.current_depth);

            tracing::trace!(
                current_depth=args.current_depth,
                depth=args.remaining_depth,
                color=?P::COLOR,
                alpha=?args.alpha,
                beta=?args.beta,
                "move"=%args.mv,
                board=%args.old_board,
                ?score,
                "{}", "eval".bright_blue()
            );

            return score;
        }

        let mut score = P::WORST_SCORE;

        for mv in moves {
            if args.timeout.is_complete() {
                break;
            }

            let new = self.alphabeta::<P::Flip>(AlphaBetaArgs {
                old_board: &board,
                mv,
                timeout: args.timeout,
                remaining_depth: args.remaining_depth - 1,
                current_depth: args.current_depth + 1,
                alpha: args.alpha,
                beta: args.beta,
            });

            let old_score = score;
            if P::is_better(score, new) {
                tracing::trace!(
                    current_depth=args.current_depth,
                    depth=args.remaining_depth,
                    color=?P::COLOR,
                    alpha=?args.alpha,
                    beta=?args.beta,
                    score.old=?old_score,
                    score.new=?new,
                    score.current=?score,
                    "move"=%args.mv,
                    board=%args.old_board,
                    "{}",
                    "better".bright_green()
                );
                score = new;
            } else {
                tracing::trace!(
                    current_depth=args.current_depth,
                    depth=args.remaining_depth,
                    color=?P::COLOR,
                    alpha=?args.alpha,
                    beta=?args.beta,
                    score.old=?old_score,
                    score.new=?new,
                    score.current=?score,
                    "move"=%args.mv,
                    board=%args.old_board,
                    "{}",
                    "worse".red()
                );
            }

            P::update_cutoff(&mut args.alpha, &mut args.beta, score);

            if args.beta <= args.alpha {
                tracing::trace!(
                    current_depth=args.current_depth,
                    depth=args.remaining_depth,
                    color=?P::COLOR,
                    alpha=?args.alpha,
                    beta=?args.beta,
                    score.old=?old_score,
                    score.new=?new,
                    score.current=?score,
                    "move"=%args.mv,
                    board=%args.old_board,
                    "{}",
                    if P::IS_BETA_CUTOFF {
                        "beta cutoff"
                    } else {
                        "alpha cutoff"
                    }.bright_green()
                );
                break;
            }
        }

        tracing::trace!(
            current_depth=args.current_depth,
            depth=args.remaining_depth,
            color=?P::COLOR,
            alpha=?args.alpha,
            beta=?args.beta,
            ?score,
            "move"=%args.mv,
            board=%args.old_board,
            "finish alpha beta"
        );
        if tracing::enabled!(tracing::Level::TRACE) {
            eprintln!();
        }
        score
    }
    fn eval(&mut self, board: &Board, current_depth: u16) -> Score {
        self.moves_evaluated += 1;

        if board.half_move_clock() >= 100 {
            return Score::Raw(0);
        }

        let white_piece_score = self.score_pieces(board, Color::White);
        let black_piece_score = self.score_pieces(board, Color::Black);

        let piece_score = white_piece_score - black_piece_score;

        let mut white_endgame_score = 0;
        let mut black_endgame_score = 0;

        // match piece_score.cmp(&0) {
        //     std::cmp::Ordering::Less => {
        //         if black_piece_score < 800 + 500 * 3 {
        //             // if we are in the endgame
        //             let white_king = board.king_sq(Color::White);
        //             let black_king = board.king_sq(Color::Black);

        //             let dist = chess_lookup::distance(white_king, black_king);
        //             // minimize the distance to the
        //             black_endgame_score -= (dist as i32) * 1000;
        //             // penalized for staying close to the edge
        //             white_endgame_score -= DIST_FROM_CENTER[white_king] as i32 * 100;
        //             // black_endgame_score -= DIST_FROM_CENTER[black_king] as i32 * 30;
        //         }
        //     }
        //     std::cmp::Ordering::Equal => (),
        //     std::cmp::Ordering::Greater => {
        //         if white_piece_score < 800 + 500 * 3 {
        //             // if we are in the endgame
        //             let white_king = board.king_sq(Color::White);
        //             let black_king = board.king_sq(Color::Black);

        //             let dist = chess_lookup::distance(white_king, black_king);
        //             tracing::trace!(current_depth, ?dist);
        //             // minimize the distance to the
        //             white_endgame_score -= (dist as i32) * (dist as i32) * 1000;
        //             // penalized for staying close to the edge
        //             // white_score -= DIST_FROM_CENTER[white_king] as i32 * 30;
        //             black_endgame_score -= DIST_FROM_CENTER[black_king] as i32 * 100;
        //         }
        //     }
        // }

        let white_score = white_piece_score * 100 + white_endgame_score;
        let black_score = black_piece_score * 100 + black_endgame_score;

        tracing::trace!(current_depth, ?white_score, ?black_score);

        let piece_score = white_score - black_score;

        Score::Raw(piece_score)
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

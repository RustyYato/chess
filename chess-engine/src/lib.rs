mod score;

use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use chess_bitboard::{BitBoard, Color, Piece};
use chess_movegen::{Board, ChessMove};
use colorz::Colorize as _;
pub use score::Score;

#[derive(Default)]
pub struct Engine {
    pub moves_evaluated: u64,
    pub max_depth: u16,
}

#[derive(Default)]
pub struct ThreeFold {
    boards: HashMap<Board, u8, IntHashBuilder>,
}

impl core::fmt::Debug for ThreeFold {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_map();

        for (board, count) in &self.boards {
            f.entry(&format_args!("{board}"), count);
        }

        f.finish()
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct IntHashBuilder;
struct IntHasher(Option<u64>);

impl std::hash::BuildHasher for IntHashBuilder {
    type Hasher = IntHasher;

    #[inline]
    fn build_hasher(&self) -> Self::Hasher {
        IntHasher(None)
    }
}

impl std::hash::Hasher for IntHasher {
    #[inline]
    fn finish(&self) -> u64 {
        self.0.unwrap()
    }

    fn write(&mut self, _bytes: &[u8]) {
        todo!()
    }

    #[inline]
    fn write_u64(&mut self, i: u64) {
        self.0 = Some(i)
    }
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

#[derive(Clone, Copy)]
struct AlphaBetaArgs<'a, T> {
    old_board: &'a Board,
    timeout: T,
    remaining_depth: u16,
    current_depth: u16,
    alpha: Score,
    beta: Score,
    list: BoardList<'a>,
}

#[derive(Clone, Copy)]
enum PrevBoard<'a> {
    Root,
    Prev(&'a BoardList<'a>),
}

#[derive(Clone, Copy)]
struct BoardList<'a> {
    prev: PrevBoard<'a>,
    three_fold: &'a ThreeFold,
    board: &'a Board,
    count: u8,
}

impl ThreeFold {
    pub fn new() -> Self {
        ThreeFold {
            boards: HashMap::with_hasher(IntHashBuilder),
        }
    }

    pub fn add(&mut self, board: Board) -> bool {
        let count = self.boards.entry(board).or_insert(0);
        *count += 1;
        *count == 3
    }

    pub fn get(&self, board: &Board) -> u8 {
        let count = *self.boards.get(board).unwrap_or(&0);
        // println!("get {count:?}");
        count
    }
}

impl<'a> BoardList<'a> {
    pub fn new(board: &'a Board, three_fold: &'a ThreeFold) -> Self {
        Self {
            prev: PrevBoard::Root,
            board,
            three_fold,
            count: three_fold.get(board),
        }
    }

    pub fn add(&'a self, board: &'a Board) -> Self {
        Self {
            prev: PrevBoard::Prev(self),
            board,
            three_fold: self.three_fold,
            count: self.count(board) + 1,
        }
    }

    pub fn count(&self, board: &Board) -> u8 {
        if self.board == board {
            self.count
        } else {
            match self.prev {
                PrevBoard::Root => self.three_fold.get(board),
                PrevBoard::Prev(prev) => prev.count(board),
            }
        }
    }
}

impl Engine {
    pub fn search(
        &mut self,
        board: &Board,
        three_fold: &ThreeFold,
        timeout: impl TimeoutRef,
    ) -> (Option<ChessMove>, Score) {
        match board.turn() {
            Color::White => self.search_with::<White>(board, three_fold, timeout),
            Color::Black => self.search_with::<Black>(board, three_fold, timeout),
        }
    }

    fn search_with<P: Policy>(
        &mut self,
        board: &Board,
        three_fold: &ThreeFold,
        timeout: impl TimeoutRef,
    ) -> (Option<ChessMove>, Score) {
        assert_eq!(P::COLOR, board.turn());
        self.moves_evaluated = 0;

        let mut best_score = P::WORST_SCORE;
        let mut best_mv = None;

        let mut depth = 0;

        loop {
            tracing::debug!(color = ?P::COLOR, depth, board=%board, "start depth");
            let mut score = P::WORST_SCORE;
            let mut best_mv_at = None;

            let mut args = AlphaBetaArgs {
                old_board: board,
                timeout,
                remaining_depth: depth,
                current_depth: 1,
                alpha: Score::Min,
                beta: Score::Max,
                list: BoardList::new(board, three_fold),
            };

            let mut moves = board.legals();

            if let Some(mv) = best_mv {
                tracing::debug!("Consider previous best move");
                moves.remove_move(mv);

                let new = self.alphabeta::<P::Flip>(mv, &args);

                if timeout.is_complete() {
                    break;
                }

                if P::is_better(score, new) {
                    score = new;
                    best_mv_at = Some(mv);
                    tracing::debug!(color = ?P::COLOR, depth, "move"=%mv, board=%board, ?score, "{}", "better".bright_green());
                }

                P::update_cutoff(&mut args.alpha, &mut args.beta, score)
            }

            // iterate over captures first
            moves.set_mask(board[!P::COLOR]);

            if !moves.is_empty() {
                tracing::debug!("Consider captures");
            }

            for mv in &mut moves {
                let new = self.alphabeta::<P::Flip>(mv, &args);

                if timeout.is_complete() {
                    break;
                }

                if P::is_better(score, new) {
                    score = new;
                    best_mv_at = Some(mv);
                    tracing::debug!(color = ?P::COLOR, depth, "move"=%mv, board=%board, ?score, "{}", "better".bright_green());
                }

                P::update_cutoff(&mut args.alpha, &mut args.beta, score)
            }

            tracing::debug!("Consider normal moves");

            // clear mask
            moves.set_mask(!BitBoard::empty());

            for mv in moves {
                let new = self.alphabeta::<P::Flip>(mv, &args);

                if timeout.is_complete() {
                    break;
                }

                if P::is_better(score, new) {
                    score = new;
                    best_mv_at = Some(mv);
                    tracing::debug!(color = ?P::COLOR, depth, "move"=%mv, board=%board, ?score, "{}", "better".bright_green());
                }

                P::update_cutoff(&mut args.alpha, &mut args.beta, score)
            }

            if timeout.is_complete() {
                tracing::info!("Timeout");
                break;
            }

            best_mv = best_mv_at;
            best_score = score;
            self.max_depth = depth;
            depth += 1;

            match score {
                Score::BlackMateIn(_) | Score::WhiteMateIn(_) => break,
                _ => (),
            }
        }

        (best_mv, best_score)
    }

    fn alphabeta<P: Policy>(
        &mut self,
        mv: ChessMove,
        args: &AlphaBetaArgs<'_, impl TimeoutRef>,
    ) -> Score {
        let board = unsafe { args.old_board.move_unchecked(mv) };
        let was_capture = args.old_board.raw().get(mv.dest).is_some();
        let list = if was_capture {
            BoardList::new(&board, args.list.three_fold)
        } else {
            args.list.add(&board)
        };

        if args.current_depth == 1 {
            tracing::debug!(
                current_depth=args.current_depth,
                depth=args.remaining_depth,
                color=?P::COLOR,
                alpha=?args.alpha,
                beta=?args.beta,
                "move"=%mv,
                was_capture,
                board=%args.old_board,
                "consider move"
            );
        } else {
            tracing::trace!(
                current_depth=args.current_depth,
                depth=args.remaining_depth,
                color=?P::COLOR,
                alpha=?args.alpha,
                beta=?args.beta,
                "move"=%mv,
                board=%args.old_board,
                "start alphabeta"
            );
        }

        if was_capture && self.insuffient_material(&board) {
            tracing::trace!(
                current_depth=args.current_depth,
                depth=args.remaining_depth,
                color=?P::COLOR,
                alpha=?args.alpha,
                beta=?args.beta,
                "move"=%mv,
                was_capture,
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
                    "move"=%mv,
                    was_capture,
                    board=%args.old_board,
                    "{}", "mate".bright_blue()
                );
                return match P::COLOR {
                    Color::White => Score::BlackMateIn(args.current_depth),
                    Color::Black => Score::WhiteMateIn(args.current_depth),
                };
            } else {
                tracing::trace!(
                    current_depth=args.current_depth,
                    depth=args.remaining_depth,
                    color=?P::COLOR,
                    alpha=?args.alpha,
                    beta=?args.beta,
                    "move"=%mv,
                    was_capture,
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
                "move"=%mv,
                was_capture,
                board=%args.old_board,
                "{}", "tie (clock)".bright_blue()
            );
            return Score::Raw(0);
        }

        if list.count == 3 {
            tracing::trace!(
                current_depth=args.current_depth,
                depth=args.remaining_depth,
                alpha=?args.alpha,
                beta=?args.beta,
                "move"=%mv,
                was_capture,
                board=%args.old_board,
                "{}", "tie (reps)".bright_blue()
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
                "move"=%mv,
                was_capture,
                board=%args.old_board,
                ?score,
                "{}", "eval".bright_blue()
            );

            return score;
        }

        let mut score = P::WORST_SCORE;

        let mut args = AlphaBetaArgs {
            old_board: &board,
            timeout: args.timeout,
            remaining_depth: args.remaining_depth - 1,
            current_depth: args.current_depth + 1,
            alpha: args.alpha,
            beta: args.beta,
            list,
        };

        for mv in moves {
            if args.timeout.is_complete() {
                break;
            }

            let new = self.alphabeta::<P::Flip>(mv, &args);

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
                    "move"=%mv,
                    "move.next"=%mv,
                    was_capture,
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
                    "move"=%mv,
                    "move.next"=%mv,
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
                    "move"=%mv,
                    was_capture,
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
            "move"=%mv,
            was_capture,
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

        match piece_score.cmp(&0) {
            std::cmp::Ordering::Less => {
                if black_piece_score < 800 + 500 * 3 {
                    white_endgame_score += self.eval_endgame(board, Color::Black)
                }
            }
            std::cmp::Ordering::Equal => (),
            std::cmp::Ordering::Greater => {
                if white_piece_score < 800 + 500 * 3 {
                    black_endgame_score += self.eval_endgame(board, Color::White)
                }
            }
        }

        let white_score = white_piece_score * 100 + white_endgame_score;
        let black_score = black_piece_score * 100 + black_endgame_score;

        tracing::trace!(current_depth, ?white_score, ?black_score);

        // assert!(white_score > black_score);

        let piece_score = white_score - black_score;

        Score::Raw(piece_score)
    }

    fn eval_endgame(&mut self, board: &Board, better: Color) -> i32 {
        // if we are in the endgame
        let better_king = board.king_sq(better);
        let worse_king = board.king_sq(!better);

        let king_moves = board.king_legals(!better).len();
        // dbg!(king_moves);

        let dist = chess_lookup::distance(better_king, worse_king);

        let mut penalty = 0;
        // penalize for staying far away from the king
        penalty += (dist as i32) * (dist as i32) * 100;
        // penalize for staying far from the edge
        penalty += DIST_FROM_EDGE[worse_king] as i32 * 10;
        // penalize for allowing more kings moves
        penalty += king_moves as i32 * 1000;

        penalty
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
static DIST_FROM_EDGE: [u8; 64] = {
    use chess_bitboard::{File, Rank, Pos};
    let mut scores = [0; 64];

    let mut i = 0;

    while i < scores.len() {
        let Some(pos) = Pos::from_u8(i as u8) else {
            unreachable!()
        };

        let file = pos.file();
        let rank = pos.rank();

        let to_a = file.dist_to(File::A);
        let to_h = file.dist_to(File::H);

        let to_1 = rank.dist_to(Rank::_1);
        let to_8 = rank.dist_to(Rank::_8);

        let to_file_edge = if to_a < to_h {
            to_a
        } else {
            to_h
        };
        let to_rank_edge = if to_1 < to_8 {
            to_1
        } else {
            to_8
        };

        scores[i] = to_file_edge * to_rank_edge * 10 + to_file_edge * to_file_edge + to_rank_edge * to_rank_edge;

        i += 1;
    }

    scores
};

#[test]
fn test() {
    for rank in chess_bitboard::Rank::all().rev() {
        for pos in rank {
            print!("{:2} ", DIST_FROM_EDGE[pos]);
        }

        println!();
    }

    panic!()
}

use abi_stable::prefix_type::PrefixTypeTrait;
use chess_api::{abi_stable::export_root_module, Board};

#[export_root_module]
fn load_api() -> chess_api::ChessApiRef {
    PrefixTypeTrait::leak_into_prefix(chess_api::ChessApi::new(|| ChessBot {
        three_fold: chess_engine::ThreeFold::new(),
        board: Board::standard(),
        engine: chess_engine::Engine::default(),
    }))
}

pub struct ChessBot {
    three_fold: chess_engine::ThreeFold,
    engine: chess_engine::Engine,
    board: Board,
}

impl chess_api::ChessEngineTrait for ChessBot {
    fn evaluate(&mut self, timeout: chess_api::TimeoutReference<'_>) -> chess_api::EvaluatedMove {
        let (mv, score) = self.engine.search(&self.board, &self.three_fold, &timeout);
        chess_api::EvaluatedMove::new(mv, score)
    }

    fn set_board(&mut self, board: Board) {
        self.board = board;
        self.three_fold = chess_engine::ThreeFold::new();
    }

    fn make_move(&mut self, mv: chess_api::StableChessMove) -> bool {
        let mv = chess_movegen::ChessMove::from(mv);
        self.board.move_mut(mv)
    }
}

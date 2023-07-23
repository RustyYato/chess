use std::time::Duration;

use chess_movegen::ChessMove;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ChessEngine {
    engine: chess_engine::Engine,
}

#[wasm_bindgen]
pub struct ChessGame {
    history: chess_engine::ThreeFold,
    board: chess_movegen::Board,
}

#[wasm_bindgen]
pub struct EngineChessMove {
    chess_move: Option<ChessMove>,
    score: chess_engine::Score,
}

#[wasm_bindgen]
impl ChessEngine {
    pub fn search(
        &mut self,
        game: &ChessGame,
        timeout: Option<String>,
    ) -> Result<EngineChessMove, wasm_bindgen::JsError> {
        let timeout = timeout
            .map(|timeout| parse_duration::parse(&timeout))
            .transpose()
            .map_err(|err| wasm_bindgen::JsError::new(&err.to_string()))?
            .unwrap_or(Duration::from_secs(5));

        let (chess_move, score) = self.engine.search(
            &game.board,
            &game.history,
            chess_engine::DurationTimeout::new(timeout),
        );

        Ok(EngineChessMove { chess_move, score })
    }
}

#[wasm_bindgen]
impl EngineChessMove {
    pub fn chess_move(&self) -> Option<String> {
        self.chess_move.map(|mv| mv.to_string())
    }
}

use tracing_wasm::{set_as_global_default_with_config, WASMLayerConfigBuilder};

#[wasm_bindgen]
pub fn init_logging() {
    console_error_panic_hook::set_once();
    set_as_global_default_with_config(
        WASMLayerConfigBuilder::new()
            .set_max_level(tracing::Level::DEBUG)
            .build(),
    )
}

#[wasm_bindgen]
pub fn new_engine() -> ChessEngine {
    ChessEngine {
        engine: chess_engine::Engine::default(),
    }
}

#[wasm_bindgen]
pub fn new_game() -> ChessGame {
    ChessGame {
        history: chess_engine::ThreeFold::new(),
        board: chess_movegen::Board::standard(),
    }
}

#[wasm_bindgen]
pub fn new_game_from_fen(s: &str) -> Result<ChessGame, JsError> {
    let board = s
        .parse::<chess_movegen::Board>()
        .map_err(|err| JsError::new(&format!("{err:?}")))?;
    Ok(ChessGame {
        history: chess_engine::ThreeFold::new(),
        board,
    })
}

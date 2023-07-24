use std::time::Duration;

use chess_bitboard::{File, Pos, Rank};
use chess_movegen::ChessMove;
use wasm_bindgen::{prelude::*, JsError};

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
    ) -> Result<EngineChessMove, JsError> {
        let timeout = timeout
            .map(|timeout| parse_duration::parse(&timeout))
            .transpose()
            .map_err(|err| JsError::new(&err.to_string()))?
            .unwrap_or(Duration::from_secs(5));

        let (chess_move, score) = self.engine.search(
            &game.board,
            &game.history,
            chess_engine::DurationTimeout::new(timeout),
        );

        Ok(EngineChessMove { chess_move, score })
    }
}

impl ChessGame {
    pub fn get(&self, file: u8, rank: u8) -> Result<u8, JsError> {
        let file = File::from_u8(file).ok_or_else(|| JsError::new("Invalid file"))?;
        let rank = Rank::from_u8(rank).ok_or_else(|| JsError::new("Invalid rank"))?;
        match self.board.raw().get(Pos::new(file, rank)) {
            Some((color, piece)) => Ok((piece as u8 + 1) << 1 | color as u8),
            None => Ok(0),
        }
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

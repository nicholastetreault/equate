pub mod board;
pub mod scoring;
pub mod tile;
pub mod validation;

pub use board::Board;
pub use tile::{Operator, PlacedTile, Tile, TileKind};
pub use validation::ValidationError;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use rand::seq::SliceRandom;

/// Full game state, managed server-side.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GameState {
    pub board: Board,
    pub bag: Vec<Tile>,
    pub is_first_move: bool,
}

impl GameState {
    pub fn new() -> Self {
        let mut bag = tile::build_tile_bag();
        bag.shuffle(&mut rand::thread_rng());
        GameState {
            board: Board::new(),
            bag,
            is_first_move: true,
        }
    }

    pub fn draw_tiles(&mut self, n: usize) -> Vec<Tile> {
        let count = n.min(self.bag.len());
        self.bag.drain(..count).collect()
    }

    /// Validate, apply, and score a move. Returns the points earned.
    pub fn apply_move(&mut self, placed: &[PlacedTile]) -> Result<u32, String> {
        validation::validate_move(&self.board, placed, self.is_first_move)
            .map_err(|e| e.to_string())?;

        let score = scoring::score_move(&self.board, placed);

        for pt in placed {
            self.board.place_tile(pt.row, pt.col, pt.tile.clone());
        }

        self.is_first_move = false;
        Ok(score)
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// WASM exports for client-side move preview
// ---------------------------------------------------------------------------

/// Validate a move without applying it. Returns an error string on failure.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn validate_move_wasm(
    board_json: &str,
    placed_json: &str,
    is_first_move: bool,
) -> Result<(), JsValue> {
    let board: Board = serde_json::from_str(board_json)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    let placed: Vec<PlacedTile> = serde_json::from_str(placed_json)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    validation::validate_move(&board, &placed, is_first_move)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Preview the score for a move without applying it.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn score_move_wasm(board_json: &str, placed_json: &str) -> u32 {
    let board: Board = serde_json::from_str(board_json).unwrap_or_default();
    let placed: Vec<PlacedTile> = serde_json::from_str(placed_json).unwrap_or_default();
    scoring::score_move(&board, &placed)
}

use serde::{Deserialize, Serialize};

use crate::tile::Tile;

pub const BOARD_SIZE: usize = 19;
pub const CENTER: usize = 9;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PremiumSquare {
    /// 2S — 2× individual symbol score
    DoubleTile,
    /// 3S — 3× individual symbol score
    TripleTile,
    /// 2E — 2× equation score
    DoubleEquation,
    /// 3E — 3× equation score
    TripleEquation,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Cell {
    pub tile: Option<Tile>,
    pub premium: Option<PremiumSquare>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Board {
    pub cells: Vec<Vec<Cell>>,
}

impl Board {
    pub fn new() -> Self {
        let mut cells = vec![vec![Cell::default(); BOARD_SIZE]; BOARD_SIZE];
        for (row, col, premium) in premium_squares() {
            cells[row][col].premium = Some(premium);
        }
        Board { cells }
    }

    pub fn place_tile(&mut self, row: usize, col: usize, tile: Tile) {
        self.cells[row][col].tile = Some(tile);
    }

    pub fn get_tile(&self, row: usize, col: usize) -> Option<&Tile> {
        self.cells.get(row)?.get(col)?.tile.as_ref()
    }

    pub fn is_occupied(&self, row: usize, col: usize) -> bool {
        self.cells
            .get(row)
            .and_then(|r| r.get(col))
            .map(|c| c.tile.is_some())
            .unwrap_or(false)
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

/// Premium square layout from the official Equate board.
/// Board is 19×19, center at (9,9) = 2E.
/// Rows 10–18 are the vertical mirror of rows 8–0.
///
/// Parsed from user-supplied matrix:
/// Row 0:  [0,3E,0,0,0,3S,0,0,0,2S,0,0,0,3S,0,0,0,3E,0]
/// Row 1:  [3E,0,0,0,3S,0,0,0,2S,0,2S,0,0,0,3S,0,0,0,3E]
/// Row 2:  [0,0,0,2E,0,0,0,2S,0,0,0,2S,0,0,0,2E,0,0,0]
/// Row 3:  [0,0,3E,0,0,0,3S,0,0,0,0,0,3S,0,0,0,2E,0,0]
/// Row 4:  [0,3S,0,0,0,2S,0,0,0,3S,0,0,0,2S,0,0,0,3S,0]
/// Row 5:  [3S,0,0,0,2S,0,0,0,3S,0,3S,0,0,0,2S,0,0,0,3S]
/// Row 6:  [0,0,0,3S,0,0,0,2S,0,0,0,2S,0,0,0,3S,0,0,0]
/// Row 7:  [0,0,2S,0,0,0,2S,0,0,0,0,0,2S,0,0,0,2S,0,0]
/// Row 8:  [0,2S,0,0,0,3S,0,0,2S,0,2S,0,0,3S,0,0,0,2S,0]
/// Row 9:  [2S,0,0,0,3S,0,0,0,0,2E,0,0,0,0,3S,0,0,0,2S]
/// Rows 10–18 mirror rows 8–0.
fn premium_squares() -> Vec<(usize, usize, PremiumSquare)> {
    use PremiumSquare::*;

    let top_half: Vec<(usize, usize, PremiumSquare)> = vec![
        // ── Row 0 ────────────────────────────────────────────────────────────
        (0, 1, TripleEquation), (0, 17, TripleEquation),
        (0, 5, TripleTile),     (0, 13, TripleTile),
        (0, 9, DoubleTile),

        // ── Row 1 ────────────────────────────────────────────────────────────
        (1, 0, TripleEquation), (1, 18, TripleEquation),
        (1, 4, TripleTile),     (1, 14, TripleTile),
        (1, 8, DoubleTile),     (1, 10, DoubleTile),

        // ── Row 2 ────────────────────────────────────────────────────────────
        (2, 3, DoubleEquation), (2, 15, DoubleEquation),
        (2, 7, DoubleTile),     (2, 11, DoubleTile),

        // ── Row 3 ────────────────────────────────────────────────────────────
        (3, 2, DoubleEquation), (3, 16, DoubleEquation),
        (3, 6, TripleTile),     (3, 12, TripleTile),

        // ── Row 4 ────────────────────────────────────────────────────────────
        (4, 1, TripleTile),     (4, 9, TripleTile),     (4, 17, TripleTile),
        (4, 5, DoubleTile),     (4, 13, DoubleTile),

        // ── Row 5 ────────────────────────────────────────────────────────────
        (5, 0, TripleTile),     (5, 8, TripleTile),
        (5, 10, TripleTile),    (5, 18, TripleTile),
        (5, 4, DoubleTile),     (5, 14, DoubleTile),

        // ── Row 6 ────────────────────────────────────────────────────────────
        (6, 3, TripleTile),     (6, 15, TripleTile),
        (6, 7, DoubleTile),     (6, 11, DoubleTile),

        // ── Row 7 ────────────────────────────────────────────────────────────
        (7, 2, DoubleTile),     (7, 6, DoubleTile),
        (7, 12, DoubleTile),    (7, 16, DoubleTile),

        // ── Row 8 ────────────────────────────────────────────────────────────
        (8, 1, DoubleTile),     (8, 8, DoubleTile),
        (8, 10, DoubleTile),    (8, 17, DoubleTile),
        (8, 5, TripleTile),     (8, 13, TripleTile),

        // ── Row 9 (center row) ───────────────────────────────────────────────
        (9, 0, DoubleTile),     (9, 18, DoubleTile),
        (9, 4, TripleTile),     (9, 14, TripleTile),
        (9, 9, DoubleEquation), // CENTER
    ];

    // Mirror rows 8–0 onto rows 10–18
    let mut all = top_half;
    for (row, col, ref premium) in all.clone() {
        if row < 9 {
            let mirrored_row = 18 - row;
            all.push((mirrored_row, col, premium.clone()));
        }
    }

    all
}

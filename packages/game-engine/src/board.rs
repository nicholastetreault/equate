use serde::{Deserialize, Serialize};

use crate::tile::Tile;

pub const BOARD_SIZE: usize = 15;
pub const CENTER: usize = 7;

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

/// Premium square layout mapped from the official Equate board image.
/// Board is 15×15, center at (7,7) = 2E.
/// Symmetric across both axes.
fn premium_squares() -> Vec<(usize, usize, PremiumSquare)> {
    use PremiumSquare::*;
    vec![
        // ── 3E (TripleEquation) — pink, outer corners ────────────────────
        (0, 0, TripleEquation),  (0, 14, TripleEquation),
        (1, 0, TripleEquation),  (1, 14, TripleEquation),
        (13, 0, TripleEquation), (13, 14, TripleEquation),
        (14, 0, TripleEquation), (14, 14, TripleEquation),

        // ── 2E (DoubleEquation) — dark purple ────────────────────────────
        (2, 2, DoubleEquation),  (2, 12, DoubleEquation),
        (3, 2, DoubleEquation),  (3, 12, DoubleEquation),
        (7, 7, DoubleEquation),  // center
        (11, 2, DoubleEquation), (11, 12, DoubleEquation),
        (12, 2, DoubleEquation), (12, 12, DoubleEquation),

        // ── 3S (TripleTile) — green ───────────────────────────────────────
        (0, 3, TripleTile),  (0, 11, TripleTile),
        (1, 3, TripleTile),  (1, 11, TripleTile),
        (3, 4, TripleTile),  (3, 10, TripleTile),
        (4, 0, TripleTile),  (4, 7, TripleTile),  (4, 14, TripleTile),
        (5, 0, TripleTile),  (5, 5, TripleTile),  (5, 9, TripleTile),  (5, 14, TripleTile),
        (7, 4, TripleTile),  (7, 10, TripleTile),
        (9, 0, TripleTile),  (9, 5, TripleTile),  (9, 9, TripleTile),  (9, 14, TripleTile),
        (10, 0, TripleTile), (10, 7, TripleTile), (10, 14, TripleTile),
        (11, 4, TripleTile), (11, 10, TripleTile),
        (13, 3, TripleTile), (13, 11, TripleTile),
        (14, 3, TripleTile), (14, 11, TripleTile),

        // ── 2S (DoubleTile) — teal/light blue ────────────────────────────
        (0, 7, DoubleTile),
        (1, 5, DoubleTile),  (1, 9, DoubleTile),
        (2, 5, DoubleTile),  (2, 9, DoubleTile),
        (4, 4, DoubleTile),  (4, 10, DoubleTile),
        (5, 3, DoubleTile),  (5, 11, DoubleTile),
        (6, 3, DoubleTile),  (6, 11, DoubleTile),
        (7, 0, DoubleTile),  (7, 14, DoubleTile),
        (8, 3, DoubleTile),  (8, 11, DoubleTile),
        (9, 3, DoubleTile),  (9, 11, DoubleTile),
        (10, 4, DoubleTile), (10, 10, DoubleTile),
        (12, 5, DoubleTile), (12, 9, DoubleTile),
        (13, 5, DoubleTile), (13, 9, DoubleTile),
        (14, 7, DoubleTile),
    ]
}

use serde::{Deserialize, Serialize};

use crate::tile::Tile;

pub const BOARD_SIZE: usize = 19;
pub const CENTER: usize = 9;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PremiumSquare {
    DoubleTile,
    TripleTile,
    DoubleEquation,
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

/// Premium square layout for the 19×19 Equate board.
fn premium_squares() -> Vec<(usize, usize, PremiumSquare)> {
    use PremiumSquare::*;
    vec![
        // Triple equation — corners and edge midpoints
        (0, 0, TripleEquation), (0, 9, TripleEquation), (0, 18, TripleEquation),
        (9, 0, TripleEquation), (9, 18, TripleEquation),
        (18, 0, TripleEquation), (18, 9, TripleEquation), (18, 18, TripleEquation),

        // Double equation
        (3, 3, DoubleEquation), (3, 15, DoubleEquation),
        (15, 3, DoubleEquation), (15, 15, DoubleEquation),
        (7, 7, DoubleEquation), (7, 11, DoubleEquation),
        (11, 7, DoubleEquation), (11, 11, DoubleEquation),

        // Triple tile
        (0, 5, TripleTile), (0, 13, TripleTile),
        (5, 0, TripleTile), (5, 18, TripleTile),
        (13, 0, TripleTile), (13, 18, TripleTile),
        (18, 5, TripleTile), (18, 13, TripleTile),

        // Double tile
        (2, 6, DoubleTile), (2, 12, DoubleTile),
        (6, 2, DoubleTile), (6, 16, DoubleTile),
        (12, 2, DoubleTile), (12, 16, DoubleTile),
        (16, 6, DoubleTile), (16, 12, DoubleTile),
        (4, 9, DoubleTile), (9, 4, DoubleTile),
        (14, 9, DoubleTile), (9, 14, DoubleTile),
    ]
}

use std::collections::HashSet;

use crate::board::{Board, PremiumSquare};
use crate::tile::PlacedTile;
use crate::validation::equation_positions;

/// Score a move according to Equate rules:
/// - Each equation formed scores ALL tiles in it (not just newly placed ones)
/// - Symbol multipliers (2S, 3S) apply only to newly placed tiles
/// - Equation multipliers (2E, 3E) from newly placed tiles multiply the full equation total
/// - If the same tile lies in two equations, it scores in both
pub fn score_move(board: &Board, placed: &[PlacedTile]) -> u32 {
    let placed_set: HashSet<(usize, usize)> =
        placed.iter().map(|pt| (pt.row, pt.col)).collect();

    // Build a temporary view so we can look up tile values for existing tiles too
    let get_tile_value = |row: usize, col: usize| -> u32 {
        // Check newly placed tiles first
        if let Some(pt) = placed.iter().find(|pt| pt.row == row && pt.col == col) {
            return pt.tile.point_value;
        }
        // Fall back to board
        board.get_tile(row, col).map(|t| t.point_value).unwrap_or(0)
    };

    let mut total = 0u32;

    for positions in equation_positions(board, placed) {
        let mut eq_tile_total = 0u32;
        let mut eq_multiplier = 1u32;

        for (row, col) in &positions {
            let base_value = get_tile_value(*row, *col);

            if placed_set.contains(&(*row, *col)) {
                // Newly placed: apply symbol multiplier from premium square
                let cell = &board.cells[*row][*col];
                let sym_mult = match &cell.premium {
                    Some(PremiumSquare::DoubleTile) => 2,
                    Some(PremiumSquare::TripleTile) => 3,
                    _ => 1,
                };
                eq_tile_total += base_value * sym_mult;

                // Accumulate equation multiplier
                match &cell.premium {
                    Some(PremiumSquare::DoubleEquation) => eq_multiplier *= 2,
                    Some(PremiumSquare::TripleEquation) => eq_multiplier *= 3,
                    _ => {}
                }
            } else {
                // Existing tile: face value only
                eq_tile_total += base_value;
            }
        }

        total += eq_tile_total * eq_multiplier;
    }

    total
}

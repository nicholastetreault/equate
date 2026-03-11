use crate::board::{Board, PremiumSquare};
use crate::tile::PlacedTile;

/// Score a move. Premium squares apply only to newly placed tiles.
/// Existing tiles in an equation contribute their face value.
pub fn score_move(board: &Board, placed: &[PlacedTile]) -> u32 {
    let mut tile_total: u32 = 0;
    let mut equation_multiplier: u32 = 1;

    for pt in placed {
        let cell = &board.cells[pt.row][pt.col];
        let tile_multiplier = match &cell.premium {
            Some(PremiumSquare::DoubleTile) => 2,
            Some(PremiumSquare::TripleTile) => 3,
            _ => 1,
        };
        tile_total += pt.tile.point_value * tile_multiplier;

        match &cell.premium {
            Some(PremiumSquare::DoubleEquation) => equation_multiplier *= 2,
            Some(PremiumSquare::TripleEquation) => equation_multiplier *= 3,
            _ => {}
        }
    }

    // TODO: also sum face values of existing tiles in affected equations.
    // For MVP, only newly placed tiles contribute to the score calculation.

    tile_total * equation_multiplier
}

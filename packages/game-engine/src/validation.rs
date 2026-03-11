use std::collections::HashSet;
use std::fmt;

use crate::board::{Board, BOARD_SIZE, CENTER};
use crate::tile::{Operator, PlacedTile, Tile, TileKind};

#[derive(Debug)]
pub enum ValidationError {
    NoTilesPlaced,
    NotInLineOrColumn,
    OverlapsExistingTile,
    DuplicatePosition,
    GapInPlacement,
    NotConnected,
    MustCoverCenter,
    InvalidEquation(String),
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NoTilesPlaced => write!(f, "No tiles placed"),
            Self::NotInLineOrColumn => write!(f, "All tiles must be in the same row or column"),
            Self::OverlapsExistingTile => write!(f, "Tile overlaps an existing tile"),
            Self::DuplicatePosition => write!(f, "Two tiles placed at the same position"),
            Self::GapInPlacement => write!(f, "Placed tiles must form a contiguous line (no gaps without existing tiles)"),
            Self::NotConnected => write!(f, "Tiles must connect to existing tiles"),
            Self::MustCoverCenter => write!(f, "First move must cover the center square"),
            Self::InvalidEquation(s) => write!(f, "Invalid equation: {s}"),
        }
    }
}

pub fn validate_move(
    board: &Board,
    placed: &[PlacedTile],
    is_first_move: bool,
) -> Result<(), ValidationError> {
    if placed.is_empty() {
        return Err(ValidationError::NoTilesPlaced);
    }

    // No duplicate positions among placed tiles
    let mut seen = HashSet::new();
    for pt in placed {
        if !seen.insert((pt.row, pt.col)) {
            return Err(ValidationError::DuplicatePosition);
        }
    }

    // All in the same row or column
    let all_same_row = placed.iter().all(|t| t.row == placed[0].row);
    let all_same_col = placed.iter().all(|t| t.col == placed[0].col);
    if !all_same_row && !all_same_col {
        return Err(ValidationError::NotInLineOrColumn);
    }

    // None overlap existing tiles
    for pt in placed {
        if board.is_occupied(pt.row, pt.col) {
            return Err(ValidationError::OverlapsExistingTile);
        }
    }

    // No gaps: every cell between the min and max position must be filled
    // (either by a placed tile or an existing board tile)
    if all_same_row {
        let row = placed[0].row;
        let min_col = placed.iter().map(|t| t.col).min().unwrap();
        let max_col = placed.iter().map(|t| t.col).max().unwrap();
        for col in min_col..=max_col {
            let from_placed = placed.iter().any(|t| t.col == col);
            if !from_placed && !board.is_occupied(row, col) {
                return Err(ValidationError::GapInPlacement);
            }
        }
    } else {
        let col = placed[0].col;
        let min_row = placed.iter().map(|t| t.row).min().unwrap();
        let max_row = placed.iter().map(|t| t.row).max().unwrap();
        for row in min_row..=max_row {
            let from_placed = placed.iter().any(|t| t.row == row);
            if !from_placed && !board.is_occupied(row, col) {
                return Err(ValidationError::GapInPlacement);
            }
        }
    }

    if is_first_move {
        if !placed.iter().any(|t| t.row == CENTER && t.col == CENTER) {
            return Err(ValidationError::MustCoverCenter);
        }
    } else {
        let connected = placed
            .iter()
            .any(|pt| is_adjacent_to_existing(board, pt.row, pt.col));
        if !connected {
            return Err(ValidationError::NotConnected);
        }
    }

    // Validate all equations formed by this move
    let temp = TempBoard { board, placed };
    let equations = collect_equations(&temp, placed, all_same_row);

    for eq in &equations {
        validate_equation(eq)?;
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Temporary board view (existing board + tiles being placed)
// ---------------------------------------------------------------------------

struct TempBoard<'a> {
    board: &'a Board,
    placed: &'a [PlacedTile],
}

impl<'a> TempBoard<'a> {
    fn get_tile(&self, row: usize, col: usize) -> Option<&Tile> {
        for pt in self.placed {
            if pt.row == row && pt.col == col {
                return Some(&pt.tile);
            }
        }
        self.board.get_tile(row, col)
    }
}

fn is_adjacent_to_existing(board: &Board, row: usize, col: usize) -> bool {
    [
        (row.wrapping_sub(1), col),
        (row + 1, col),
        (row, col.wrapping_sub(1)),
        (row, col + 1),
    ]
    .iter()
    .any(|&(r, c)| r < BOARD_SIZE && c < BOARD_SIZE && board.is_occupied(r, c))
}

// ---------------------------------------------------------------------------
// Equation extraction
// ---------------------------------------------------------------------------

/// Collect all contiguous tile sequences that include at least one newly placed tile.
fn collect_equations(
    temp: &TempBoard,
    placed: &[PlacedTile],
    horizontal_move: bool,
) -> Vec<Vec<TileKind>> {
    let mut equations = Vec::new();

    // Main-axis run
    let main = if horizontal_move {
        let row = placed[0].row;
        let min_col = placed.iter().map(|t| t.col).min().unwrap();
        let max_col = placed.iter().map(|t| t.col).max().unwrap();
        let start = expand_left(temp, row, min_col, true);
        let end = expand_right(temp, row, max_col, true);
        (start..=end)
            .filter_map(|c| temp.get_tile(row, c).map(|t| t.kind.clone()))
            .collect::<Vec<_>>()
    } else {
        let col = placed[0].col;
        let min_row = placed.iter().map(|t| t.row).min().unwrap();
        let max_row = placed.iter().map(|t| t.row).max().unwrap();
        let start = expand_left(temp, col, min_row, false);
        let end = expand_right(temp, col, max_row, false);
        (start..=end)
            .filter_map(|r| temp.get_tile(r, col).map(|t| t.kind.clone()))
            .collect::<Vec<_>>()
    };
    if main.len() > 1 {
        equations.push(main);
    }

    // Cross-axis runs through each newly placed tile
    for pt in placed {
        let cross = if horizontal_move {
            let start = expand_left(temp, pt.col, pt.row, false);
            let end = expand_right(temp, pt.col, pt.row, false);
            (start..=end)
                .filter_map(|r| temp.get_tile(r, pt.col).map(|t| t.kind.clone()))
                .collect::<Vec<_>>()
        } else {
            let start = expand_left(temp, pt.row, pt.col, true);
            let end = expand_right(temp, pt.row, pt.col, true);
            (start..=end)
                .filter_map(|c| temp.get_tile(pt.row, c).map(|t| t.kind.clone()))
                .collect::<Vec<_>>()
        };
        if cross.len() > 1 {
            equations.push(cross);
        }
    }

    equations
}

/// Expand left along a row (horizontal=true) or column (horizontal=false).
fn expand_left(temp: &TempBoard, fixed: usize, anchor: usize, horizontal: bool) -> usize {
    let mut pos = anchor;
    while pos > 0 {
        let (r, c) = if horizontal { (fixed, pos - 1) } else { (pos - 1, fixed) };
        if temp.get_tile(r, c).is_some() {
            pos -= 1;
        } else {
            break;
        }
    }
    pos
}

fn expand_right(temp: &TempBoard, fixed: usize, anchor: usize, horizontal: bool) -> usize {
    let mut pos = anchor;
    loop {
        let (r, c) = if horizontal { (fixed, pos + 1) } else { (pos + 1, fixed) };
        if r < BOARD_SIZE && c < BOARD_SIZE && temp.get_tile(r, c).is_some() {
            pos += 1;
        } else {
            break;
        }
    }
    pos
}

// ---------------------------------------------------------------------------
// Equation validation
// ---------------------------------------------------------------------------

enum Token {
    Number(f64),
    Op(Operator),
}

/// Tokenize tiles into numbers and operators.
/// Consecutive Number tiles form multi-digit numbers (e.g. 1,2 → 12).
fn tokenize(tiles: &[TileKind]) -> Option<Vec<Token>> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut i = 0;

    while i < tiles.len() {
        match &tiles[i] {
            TileKind::Number(n) => {
                let mut num = *n as f64;
                while i + 1 < tiles.len() {
                    if let TileKind::Number(next) = tiles[i + 1] {
                        num = num * 10.0 + next as f64;
                        i += 1;
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Number(num));
            }
            TileKind::Operator(op) => tokens.push(Token::Op(op.clone())),
            TileKind::Equals => return None, // = is not a valid token in an expression
        }
        i += 1;
    }

    Some(tokens)
}

/// Evaluate a simple arithmetic expression with correct operator precedence (* / before + -).
fn evaluate_expression(tiles: &[TileKind]) -> Option<f64> {
    let tokens = tokenize(tiles)?;

    let mut values: Vec<f64> = Vec::new();
    let mut ops: Vec<Operator> = Vec::new();

    for token in &tokens {
        match token {
            Token::Number(n) => values.push(*n),
            Token::Op(op) => ops.push(op.clone()),
        }
    }

    if values.len() != ops.len() + 1 {
        return None;
    }

    // First pass: * and /
    let mut i = 0;
    while i < ops.len() {
        match &ops[i] {
            Operator::Multiply => {
                let result = values[i] * values[i + 1];
                values[i] = result;
                values.remove(i + 1);
                ops.remove(i);
            }
            Operator::Divide => {
                if values[i + 1] == 0.0 {
                    return None;
                }
                let result = values[i] / values[i + 1];
                values[i] = result;
                values.remove(i + 1);
                ops.remove(i);
            }
            _ => i += 1,
        }
    }

    // Second pass: + and -
    let mut result = values[0];
    for (i, op) in ops.iter().enumerate() {
        match op {
            Operator::Add => result += values[i + 1],
            Operator::Subtract => result -= values[i + 1],
            _ => return None,
        }
    }

    Some(result)
}

fn validate_equation(tiles: &[TileKind]) -> Result<(), ValidationError> {
    let eq_positions: Vec<usize> = tiles
        .iter()
        .enumerate()
        .filter(|(_, t)| matches!(t, TileKind::Equals))
        .map(|(i, _)| i)
        .collect();

    if eq_positions.len() != 1 {
        return Err(ValidationError::InvalidEquation(format!(
            "must have exactly one = sign (found {})",
            eq_positions.len()
        )));
    }

    let eq_pos = eq_positions[0];
    let lhs = &tiles[..eq_pos];
    let rhs = &tiles[eq_pos + 1..];

    if lhs.is_empty() || rhs.is_empty() {
        return Err(ValidationError::InvalidEquation(
            "empty side of equation".into(),
        ));
    }

    let lhs_val = evaluate_expression(lhs)
        .ok_or_else(|| ValidationError::InvalidEquation("invalid left-hand expression".into()))?;
    let rhs_val = evaluate_expression(rhs)
        .ok_or_else(|| ValidationError::InvalidEquation("invalid right-hand expression".into()))?;

    if (lhs_val - rhs_val).abs() > f64::EPSILON * 1000.0 {
        return Err(ValidationError::InvalidEquation(format!(
            "{lhs_val} ≠ {rhs_val}"
        )));
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tile::{Operator, Tile, TileKind};

    fn num(n: u8) -> TileKind { TileKind::Number(n) }
    fn op(o: Operator) -> TileKind { TileKind::Operator(o) }
    fn eq() -> TileKind { TileKind::Equals }

    fn placed(tile: Tile, row: usize, col: usize) -> PlacedTile {
        PlacedTile { tile, row, col }
    }

    #[test]
    fn valid_simple_equation() {
        let tiles = vec![num(3), op(Operator::Add), num(4), eq(), num(7)];
        assert!(validate_equation(&tiles).is_ok());
    }

    #[test]
    fn invalid_equation() {
        let tiles = vec![num(3), op(Operator::Add), num(4), eq(), num(8)];
        assert!(validate_equation(&tiles).is_err());
    }

    #[test]
    fn multi_digit_numbers() {
        // 12 + 3 = 15
        let tiles = vec![num(1), num(2), op(Operator::Add), num(3), eq(), num(1), num(5)];
        assert!(validate_equation(&tiles).is_ok());
    }

    #[test]
    fn operator_precedence() {
        // 2 + 3 * 4 = 14
        let tiles = vec![num(2), op(Operator::Add), num(3), op(Operator::Multiply), num(4), eq(), num(1), num(4)];
        assert!(validate_equation(&tiles).is_ok());
    }

    #[test]
    fn first_move_must_cover_center() {
        let board = Board::new();
        let tiles = vec![placed(Tile::number(1), 0, 0)];
        assert!(matches!(
            validate_move(&board, &tiles, true),
            Err(ValidationError::MustCoverCenter)
        ));
    }
}

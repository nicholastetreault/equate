use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TileKind {
    Number(u8),
    Operator(Operator),
    Equals,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tile {
    pub kind: TileKind,
    pub point_value: u32,
}

impl Tile {
    pub fn number(n: u8) -> Self {
        let point_value = match n {
            0 => 3,
            1 => 1,
            2 => 1,
            3 => 2,
            4 => 2,
            5 => 2,
            6 => 2,
            7 => 3,
            8 => 3,
            9 => 4,
            _ => 1,
        };
        Tile { kind: TileKind::Number(n), point_value }
    }

    pub fn operator(op: Operator) -> Self {
        let point_value = match op {
            Operator::Add => 1,
            Operator::Subtract => 2,
            Operator::Multiply => 3,
            Operator::Divide => 5,
        };
        Tile { kind: TileKind::Operator(op), point_value }
    }

    pub fn equals() -> Self {
        Tile { kind: TileKind::Equals, point_value: 3 }
    }
}

/// A tile placed at a specific board position during a move.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlacedTile {
    pub tile: Tile,
    pub row: usize,
    pub col: usize,
}

/// Build the standard tile bag (unsorted; caller should shuffle).
pub fn build_tile_bag() -> Vec<Tile> {
    let mut bag = Vec::new();

    // (digit, count) pairs
    let numbers: &[(u8, usize)] = &[
        (0, 4), (1, 8), (2, 8), (3, 6), (4, 6),
        (5, 6), (6, 6), (7, 4), (8, 4), (9, 4),
    ];
    for &(n, count) in numbers {
        for _ in 0..count {
            bag.push(Tile::number(n));
        }
    }

    for _ in 0..8 { bag.push(Tile::operator(Operator::Add)); }
    for _ in 0..6 { bag.push(Tile::operator(Operator::Subtract)); }
    for _ in 0..6 { bag.push(Tile::operator(Operator::Multiply)); }
    for _ in 0..4 { bag.push(Tile::operator(Operator::Divide)); }
    for _ in 0..8 { bag.push(Tile::equals()); }

    bag
}

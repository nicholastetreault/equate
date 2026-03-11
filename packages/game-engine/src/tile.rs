use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

/// TileKind uses internally-tagged serde so it serializes cleanly for the client.
/// e.g. {"type":"Number","value":5}, {"type":"Fraction","numerator":1,"denominator":2}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum TileKind {
    Number { value: u8 },
    Fraction { numerator: u8, denominator: u8 },
    Operator { op: Operator },
    Equals,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tile {
    pub kind: TileKind,
    pub point_value: u32,
}

impl Tile {
    pub fn number(n: u8) -> Self {
        // Point values verified from the rulebook example (24÷3=8 → 2=1pt, 4=1pt, 3=1pt, 8=2pt)
        let point_value = match n {
            0 => 3,
            1 | 2 | 3 | 4 => 1,
            5 | 6 => 2,
            7 => 3,
            8 => 2,
            9 => 4,
            _ => 1,
        };
        Tile { kind: TileKind::Number { value: n }, point_value }
    }

    pub fn fraction(numerator: u8, denominator: u8) -> Self {
        // Higher-denominator fractions are rarer and worth more
        let point_value = match denominator {
            2 => if numerator == 1 { 2 } else { 1 },
            3 => 4,
            4 => match numerator { 1 | 3 => 3, _ => 2 },
            6 => match numerator { 1 | 5 => 5, 2 | 4 => 4, _ => 3 },
            _ => 3,
        };
        Tile { kind: TileKind::Fraction { numerator, denominator }, point_value }
    }

    pub fn operator(op: Operator) -> Self {
        let point_value = match op {
            Operator::Add => 1,
            Operator::Subtract => 2,
            Operator::Multiply => 3,
            Operator::Divide => 5,
        };
        Tile { kind: TileKind::Operator { op }, point_value }
    }

    pub fn equals() -> Self {
        Tile { kind: TileKind::Equals, point_value: 0 }
    }
}

/// A tile placed at a specific board position during a move.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlacedTile {
    pub tile: Tile,
    pub row: usize,
    pub col: usize,
}

/// Build the tile bag. Equals tiles are excluded — they are always available separately.
/// Distribution matches the Original Tile Set (190 tiles total):
///   44 operations, 103 numbers (84 whole + 19 fractions), 40 equals (separate), 3 blanks (omitted for MVP)
pub fn build_tile_bag() -> Vec<Tile> {
    let mut bag = Vec::new();

    // Whole numbers: 0×3, 1-9×9 each
    bag.extend(std::iter::repeat_with(|| Tile::number(0)).take(3));
    for n in 1u8..=9 {
        bag.extend(std::iter::repeat_with(move || Tile::number(n)).take(9));
    }

    // Fractions (19 total)
    // Halves
    for _ in 0..4 { bag.push(Tile::fraction(1, 2)); }
    for _ in 0..1 { bag.push(Tile::fraction(2, 2)); }
    // Thirds
    for _ in 0..2 { bag.push(Tile::fraction(1, 3)); }
    for _ in 0..2 { bag.push(Tile::fraction(2, 3)); }
    // Fourths
    for _ in 0..2 { bag.push(Tile::fraction(1, 4)); }
    for _ in 0..2 { bag.push(Tile::fraction(2, 4)); }
    for _ in 0..2 { bag.push(Tile::fraction(3, 4)); }
    for _ in 0..1 { bag.push(Tile::fraction(4, 4)); }
    // Sixths
    for _ in 0..1 { bag.push(Tile::fraction(1, 6)); }
    for _ in 0..1 { bag.push(Tile::fraction(2, 6)); }
    for _ in 0..1 { bag.push(Tile::fraction(3, 6)); }

    // Operations: 11 each
    for _ in 0..11 { bag.push(Tile::operator(Operator::Add)); }
    for _ in 0..11 { bag.push(Tile::operator(Operator::Subtract)); }
    for _ in 0..11 { bag.push(Tile::operator(Operator::Multiply)); }
    for _ in 0..11 { bag.push(Tile::operator(Operator::Divide)); }

    bag
}

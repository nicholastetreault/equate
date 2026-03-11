// Mirror the Rust game-engine types serialized over the wire.

export type TileKind =
  | { type: 'Number'; value: number }
  | { type: 'Operator'; op: 'Add' | 'Subtract' | 'Multiply' | 'Divide' }
  | { type: 'Equals' }

export interface Tile {
  kind: TileKind
  point_value: number
}

export interface PlacedTile {
  tile: Tile
  row: number
  col: number
}

export type PremiumSquare = 'DoubleTile' | 'TripleTile' | 'DoubleEquation' | 'TripleEquation'

export interface Cell {
  tile: Tile | null
  premium: PremiumSquare | null
}

export interface Board {
  cells: Cell[][]
}

export interface PlayerInfo {
  id: string
  name: string
}

export interface PlayerScore {
  player_id: string
  player_name: string
  score: number
}

// ── WebSocket message types ──────────────────────────────────────────────────

export type ServerMessage =
  | { type: 'room_joined'; room_code: string; player_id: string }
  | { type: 'waiting_for_opponent' }
  | {
      type: 'game_started'
      board: Board
      your_rack: Tile[]
      players: PlayerInfo[]
      current_player: string
    }
  | {
      type: 'move_accepted'
      board: Board
      scores: PlayerScore[]
      next_player: string
      your_new_rack: Tile[] | null
    }
  | { type: 'move_rejected'; reason: string }
  | { type: 'turn_changed'; current_player: string }
  | { type: 'game_over'; scores: PlayerScore[]; winner: string }
  | { type: 'error'; message: string }

export type ClientMessage =
  | { type: 'place_tiles'; tiles: PlacedTile[] }
  | { type: 'exchange_tiles'; indices: number[] }
  | { type: 'pass_turn' }

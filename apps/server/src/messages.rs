use serde::{Deserialize, Serialize};

use game_engine::{Board, PlacedTile, Tile};

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ClientMessage {
    PlaceTiles { tiles: Vec<PlacedTile> },
    ExchangeTiles { indices: Vec<usize> },
    PassTurn,
}

#[derive(Debug, Serialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServerMessage {
    RoomJoined {
        room_code: String,
        player_id: String,
    },
    WaitingForOpponent,
    GameStarted {
        board: Board,
        your_rack: Vec<Tile>,
        players: Vec<PlayerInfo>,
        current_player: String,
    },
    MoveAccepted {
        board: Board,
        scores: Vec<PlayerScore>,
        next_player: String,
        your_new_rack: Option<Vec<Tile>>,
    },
    MoveRejected {
        reason: String,
    },
    TurnChanged {
        current_player: String,
    },
    GameOver {
        scores: Vec<PlayerScore>,
        winner: String,
    },
    Error {
        message: String,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlayerInfo {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct PlayerScore {
    pub player_id: String,
    pub player_name: String,
    pub score: u32,
}

use tokio::sync::broadcast;

use game_engine::{GameState, Tile};

use crate::messages::ServerMessage;

pub const BROADCAST_CAPACITY: usize = 32;

#[derive(Debug, Clone)]
pub struct Player {
    pub id: String,
    pub name: String,
    pub rack: Vec<Tile>,
    pub score: u32,
}

#[derive(Debug)]
pub struct Room {
    pub code: String,
    pub players: Vec<Player>,
    pub game_state: Option<GameState>,
    pub current_player_idx: usize,
    pub ws_connected: usize,
    pub tx: broadcast::Sender<ServerMessage>,
}

impl Room {
    pub fn new(code: String) -> Self {
        let (tx, _) = broadcast::channel(BROADCAST_CAPACITY);
        Room {
            code,
            players: Vec::new(),
            game_state: None,
            current_player_idx: 0,
            ws_connected: 0,
            tx,
        }
    }

    /// Add a player. Returns false if the room is full or game is in progress.
    pub fn add_player(&mut self, id: String, name: String) -> bool {
        if self.players.len() >= 4 || self.game_state.is_some() {
            return false;
        }
        self.players.push(Player { id, name, rack: Vec::new(), score: 0 });
        true
    }

    pub fn has_player(&self, player_id: &str) -> bool {
        self.players.iter().any(|p| p.id == player_id)
    }

    pub fn should_start(&self) -> bool {
        self.players.len() >= 2 && self.game_state.is_none() && self.ws_connected >= 2
    }

    pub fn start_game(&mut self) {
        let mut state = GameState::new();
        for player in &mut self.players {
            player.rack = state.draw_tiles(7);
        }
        self.game_state = Some(state);
    }

    pub fn current_player_id(&self) -> Option<&str> {
        self.players.get(self.current_player_idx).map(|p| p.id.as_str())
    }

    pub fn advance_turn(&mut self) {
        self.current_player_idx = (self.current_player_idx + 1) % self.players.len();
    }

    pub fn scores(&self) -> Vec<crate::messages::PlayerScore> {
        self.players
            .iter()
            .map(|p| crate::messages::PlayerScore {
                player_id: p.id.clone(),
                player_name: p.name.clone(),
                score: p.score,
            })
            .collect()
    }
}

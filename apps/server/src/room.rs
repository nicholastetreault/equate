use std::collections::HashMap;

use tokio::sync::mpsc;

use game_engine::{GameState, Tile};

use crate::messages::ServerMessage;

#[derive(Debug, Clone)]
pub struct Player {
    pub id: String,
    pub name: String,
    pub rack: Vec<Tile>,
    pub score: u32,
}

pub struct Room {
    pub code: String,
    pub players: Vec<Player>,
    pub game_state: Option<GameState>,
    pub current_player_idx: usize,
    ws_connections: HashMap<String, mpsc::UnboundedSender<ServerMessage>>,
}

impl Room {
    pub fn new(code: String) -> Self {
        Room {
            code,
            players: Vec::new(),
            game_state: None,
            current_player_idx: 0,
            ws_connections: HashMap::new(),
        }
    }

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

    /// Register a WebSocket connection for a player. Returns the receiver to forward to that socket.
    pub fn register_ws(&mut self, player_id: String) -> mpsc::UnboundedReceiver<ServerMessage> {
        let (tx, rx) = mpsc::unbounded_channel();
        self.ws_connections.insert(player_id, tx);
        rx
    }

    pub fn unregister_ws(&mut self, player_id: &str) {
        self.ws_connections.remove(player_id);
    }

    /// True when all registered players have an active WebSocket and the game hasn't started.
    pub fn should_start(&self) -> bool {
        self.players.len() >= 2
            && self.game_state.is_none()
            && self.ws_connections.len() >= self.players.len()
    }

    pub fn start_game(&mut self) {
        let mut state = GameState::new();
        for player in &mut self.players {
            player.rack = state.draw_tiles(7);
        }
        self.game_state = Some(state);
    }

    /// Send the same message to every connected player.
    pub fn broadcast(&self, msg: ServerMessage) {
        for sender in self.ws_connections.values() {
            let _ = sender.send(msg.clone());
        }
    }

    /// Send a message to one specific player.
    pub fn send_to(&self, player_id: &str, msg: ServerMessage) {
        if let Some(sender) = self.ws_connections.get(player_id) {
            let _ = sender.send(msg);
        }
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

    pub fn player_rack(&self, player_id: &str) -> Vec<Tile> {
        self.players
            .iter()
            .find(|p| p.id == player_id)
            .map(|p| p.rack.clone())
            .unwrap_or_default()
    }
}

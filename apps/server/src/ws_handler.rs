use axum::{
    extract::{Path, State, WebSocketUpgrade},
    response::Response,
};
use axum::extract::ws::{Message, WebSocket};
use futures_util::{SinkExt, StreamExt};

use crate::{
    messages::{ClientMessage, PlayerInfo, ServerMessage},
    RoomMap,
};

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Path((room_code, player_id)): Path<(String, String)>,
    State(rooms): State<RoomMap>,
) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, room_code, player_id, rooms))
}

async fn handle_socket(
    socket: WebSocket,
    room_code: String,
    player_id: String,
    rooms: RoomMap,
) {
    let (mut sender, mut receiver) = socket.split();

    // Register connection and subscribe to room broadcasts
    let (rx, initial_msg) = {
        let mut rooms_w = rooms.write().await;
        let room = match rooms_w.get_mut(&room_code) {
            Some(r) => r,
            None => {
                let _ = sender
                    .send(Message::Text(
                        serde_json::to_string(&ServerMessage::Error {
                            message: "Room not found".into(),
                        })
                        .unwrap(),
                    ))
                    .await;
                return;
            }
        };

        if !room.has_player(&player_id) {
            let _ = sender
                .send(Message::Text(
                    serde_json::to_string(&ServerMessage::Error {
                        message: "Player not in room".into(),
                    })
                    .unwrap(),
                ))
                .await;
            return;
        }

        room.ws_connected += 1;
        let rx = room.tx.subscribe();

        // Determine what to send immediately after connecting
        let initial = if room.game_state.is_some() {
            // Game already started — send current state to this player
            let state = room.game_state.as_ref().unwrap();
            let rack = room
                .players
                .iter()
                .find(|p| p.id == player_id)
                .map(|p| p.rack.clone())
                .unwrap_or_default();
            let players: Vec<PlayerInfo> = room
                .players
                .iter()
                .map(|p| PlayerInfo { id: p.id.clone(), name: p.name.clone() })
                .collect();
            Some(ServerMessage::GameStarted {
                board: state.board.clone(),
                your_rack: rack,
                players,
                current_player: room.current_player_id().unwrap_or("").to_string(),
            })
        } else if room.should_start() {
            // This connection brings us to 2 players — start the game
            room.start_game();
            let state = room.game_state.as_ref().unwrap();
            let players: Vec<PlayerInfo> = room
                .players
                .iter()
                .map(|p| PlayerInfo { id: p.id.clone(), name: p.name.clone() })
                .collect();
            let current = room.current_player_id().unwrap_or("").to_string();

            // Broadcast GameStarted with each player's individual rack via the channel
            for player in &room.players {
                let msg = ServerMessage::GameStarted {
                    board: state.board.clone(),
                    your_rack: player.rack.clone(),
                    players: players.clone(),
                    current_player: current.clone(),
                };
                let _ = room.tx.send(msg);
            }
            None // already broadcast above
        } else {
            Some(ServerMessage::WaitingForOpponent)
        };

        (rx, initial)
    };

    // Send the initial message if any
    if let Some(msg) = initial_msg {
        let text = serde_json::to_string(&msg).unwrap();
        if sender.send(Message::Text(text)).await.is_err() {
            cleanup(&rooms, &room_code).await;
            return;
        }
    }

    // Forward broadcast messages to this client
    let mut rx = rx;
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            let text = serde_json::to_string(&msg).unwrap();
            if sender.send(Message::Text(text)).await.is_err() {
                break;
            }
        }
    });

    // Handle incoming client messages
    let rooms_clone = rooms.clone();
    let room_code_clone = room_code.clone();
    let player_id_clone = player_id.clone();
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if let Message::Text(text) = msg {
                if let Ok(client_msg) = serde_json::from_str::<ClientMessage>(&text) {
                    handle_client_message(
                        client_msg,
                        &room_code_clone,
                        &player_id_clone,
                        &rooms_clone,
                    )
                    .await;
                }
            }
        }
    });

    tokio::select! {
        _ = &mut send_task => recv_task.abort(),
        _ = &mut recv_task => send_task.abort(),
    }

    cleanup(&rooms, &room_code).await;
}

async fn cleanup(rooms: &RoomMap, room_code: &str) {
    let mut rooms_w = rooms.write().await;
    if let Some(room) = rooms_w.get_mut(room_code) {
        room.ws_connected = room.ws_connected.saturating_sub(1);
    }
}

async fn handle_client_message(
    msg: ClientMessage,
    room_code: &str,
    player_id: &str,
    rooms: &RoomMap,
) {
    match msg {
        ClientMessage::PlaceTiles { tiles } => {
            let mut rooms_w = rooms.write().await;
            let room = match rooms_w.get_mut(room_code) {
                Some(r) => r,
                None => return,
            };

            if room.current_player_id() != Some(player_id) {
                let _ = room.tx.send(ServerMessage::MoveRejected {
                    reason: "Not your turn".into(),
                });
                return;
            }

            // Apply the move and collect results while holding the game_state borrow,
            // then release it before calling methods that need &mut room again.
            let move_result = match &mut room.game_state {
                Some(state) => match state.apply_move(&tiles) {
                    Ok(score) => {
                        let new_rack = state.draw_tiles(tiles.len());
                        let board = state.board.clone();
                        Ok((score, new_rack, board))
                    }
                    Err(reason) => Err(reason),
                },
                None => return,
            };

            match move_result {
                Ok((score, new_rack, board)) => {
                    if let Some(player) = room.players.iter_mut().find(|p| p.id == player_id) {
                        player.score += score;
                        player.rack = new_rack.clone();
                    }
                    room.advance_turn();
                    let scores = room.scores();
                    let next_player = room.current_player_id().unwrap_or("").to_string();

                    let _ = room.tx.send(ServerMessage::MoveAccepted {
                        board,
                        scores,
                        next_player,
                        your_new_rack: Some(new_rack),
                    });
                }
                Err(reason) => {
                    let _ = room.tx.send(ServerMessage::MoveRejected { reason });
                }
            }
        }

        ClientMessage::PassTurn => {
            let mut rooms_w = rooms.write().await;
            let room = match rooms_w.get_mut(room_code) {
                Some(r) => r,
                None => return,
            };
            if room.current_player_id() == Some(player_id) {
                room.advance_turn();
                let next = room.current_player_id().unwrap_or("").to_string();
                let _ = room.tx.send(ServerMessage::TurnChanged { current_player: next });
            }
        }

        ClientMessage::ExchangeTiles { indices: _ } => {
            // TODO: implement tile exchange
        }
    }
}

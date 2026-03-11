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
    let (mut ws_sender, mut ws_receiver) = socket.split();

    // Register this connection and queue any initial messages before releasing the lock.
    let rx = {
        let mut rooms_w = rooms.write().await;
        let room = match rooms_w.get_mut(&room_code) {
            Some(r) => r,
            None => {
                let _ = ws_sender
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
            let _ = ws_sender
                .send(Message::Text(
                    serde_json::to_string(&ServerMessage::Error {
                        message: "Player not in room".into(),
                    })
                    .unwrap(),
                ))
                .await;
            return;
        }

        // Register gives us the receiver end of this player's private channel.
        let rx = room.register_ws(player_id.clone());

        if room.game_state.is_some() {
            // Reconnecting mid-game — send current state to just this player.
            let state = room.game_state.as_ref().unwrap();
            let players = player_infos(room);
            room.send_to(
                &player_id,
                ServerMessage::GameStarted {
                    board: state.board.clone(),
                    your_rack: room.player_rack(&player_id),
                    players,
                    current_player: room.current_player_id().unwrap_or("").to_string(),
                },
            );
        } else if room.should_start() {
            // All players connected — start the game.
            room.start_game();
            let state = room.game_state.as_ref().unwrap();
            let players = player_infos(room);
            let current = room.current_player_id().unwrap_or("").to_string();
            let board = state.board.clone();

            // Send each player their own GameStarted with their own rack.
            let racks: Vec<(String, Vec<_>)> = room
                .players
                .iter()
                .map(|p| (p.id.clone(), p.rack.clone()))
                .collect();

            for (pid, rack) in racks {
                room.send_to(
                    &pid,
                    ServerMessage::GameStarted {
                        board: board.clone(),
                        your_rack: rack,
                        players: players.clone(),
                        current_player: current.clone(),
                    },
                );
            }
        } else {
            room.send_to(&player_id, ServerMessage::WaitingForOpponent);
        }

        rx
    };

    // Forward messages from this player's channel to their WebSocket.
    let mut rx = rx;
    let mut send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            let text = serde_json::to_string(&msg).unwrap();
            if ws_sender.send(Message::Text(text)).await.is_err() {
                break;
            }
        }
    });

    // Handle messages coming in from this player's WebSocket.
    let rooms_clone = rooms.clone();
    let room_code_clone = room_code.clone();
    let player_id_clone = player_id.clone();
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = ws_receiver.next().await {
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

    let mut rooms_w = rooms.write().await;
    if let Some(room) = rooms_w.get_mut(&room_code) {
        room.unregister_ws(&player_id);
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
                room.send_to(player_id, ServerMessage::MoveRejected {
                    reason: "Not your turn".into(),
                });
                return;
            }

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

                    // Send the mover their new rack; everyone else gets None.
                    let player_ids: Vec<String> =
                        room.players.iter().map(|p| p.id.clone()).collect();

                    for pid in &player_ids {
                        let rack = if pid == player_id { Some(new_rack.clone()) } else { None };
                        room.send_to(
                            pid,
                            ServerMessage::MoveAccepted {
                                board: board.clone(),
                                scores: scores.clone(),
                                next_player: next_player.clone(),
                                your_new_rack: rack,
                            },
                        );
                    }
                }
                Err(reason) => {
                    room.send_to(player_id, ServerMessage::MoveRejected { reason });
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
                room.broadcast(ServerMessage::TurnChanged { current_player: next });
            }
        }

        ClientMessage::ExchangeTiles { indices: _ } => {
            // TODO: implement tile exchange
        }
    }
}

fn player_infos(room: &crate::room::Room) -> Vec<PlayerInfo> {
    room.players
        .iter()
        .map(|p| PlayerInfo { id: p.id.clone(), name: p.name.clone() })
        .collect()
}

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{room::Room, RoomMap};

#[derive(Deserialize)]
pub struct CreateRoomRequest {
    pub player_name: String,
}

#[derive(Serialize)]
pub struct RoomResponse {
    pub room_code: String,
    pub player_id: String,
}

pub async fn create_room(
    State(rooms): State<RoomMap>,
    Json(req): Json<CreateRoomRequest>,
) -> Result<Json<RoomResponse>, StatusCode> {
    let room_code: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .collect::<String>()
        .to_uppercase();

    let player_id = Uuid::new_v4().to_string();

    let mut room = Room::new(room_code.clone());
    room.add_player(player_id.clone(), req.player_name);

    rooms.write().await.insert(room_code.clone(), room);

    Ok(Json(RoomResponse { room_code, player_id }))
}

#[derive(Deserialize)]
pub struct JoinRoomRequest {
    pub player_name: String,
}

pub async fn join_room(
    State(rooms): State<RoomMap>,
    Path(code): Path<String>,
    Json(req): Json<JoinRoomRequest>,
) -> Result<Json<RoomResponse>, StatusCode> {
    let player_id = Uuid::new_v4().to_string();

    let mut rooms = rooms.write().await;
    let room = rooms.get_mut(&code).ok_or(StatusCode::NOT_FOUND)?;

    if !room.add_player(player_id.clone(), req.player_name) {
        return Err(StatusCode::CONFLICT);
    }

    Ok(Json(RoomResponse { room_code: code, player_id }))
}

#[derive(Serialize)]
pub struct RoomInfo {
    pub code: String,
    pub player_count: usize,
    pub in_progress: bool,
}

pub async fn get_room(
    State(rooms): State<RoomMap>,
    Path(code): Path<String>,
) -> Result<Json<RoomInfo>, StatusCode> {
    let rooms = rooms.read().await;
    let room = rooms.get(&code).ok_or(StatusCode::NOT_FOUND)?;
    Ok(Json(RoomInfo {
        code: room.code.clone(),
        player_count: room.players.len(),
        in_progress: room.game_state.is_some(),
    }))
}

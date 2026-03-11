mod messages;
mod room;
mod routes;
mod ws_handler;

use std::{collections::HashMap, sync::Arc};

use axum::{routing::get, Router};
use tokio::sync::RwLock;
use tower_http::cors::CorsLayer;

use room::Room;

pub type RoomMap = Arc<RwLock<HashMap<String, Room>>>;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let rooms: RoomMap = Arc::new(RwLock::new(HashMap::new()));

    let app = Router::new()
        .route("/api/rooms", axum::routing::post(routes::create_room))
        .route("/api/rooms/:code", get(routes::get_room))
        .route("/api/rooms/:code/join", axum::routing::post(routes::join_room))
        .route("/ws/:room_code/:player_id", get(ws_handler::ws_handler))
        .layer(CorsLayer::permissive())
        .with_state(rooms);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    tracing::info!("Server listening on http://0.0.0.0:3001");
    axum::serve(listener, app).await.unwrap();
}

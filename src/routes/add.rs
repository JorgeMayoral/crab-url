use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};
use redis::Commands;

use crate::app::AppState;

#[derive(serde::Deserialize)]
pub struct AddUrlBody {
    url: String,
}

pub async fn add_url(
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<AddUrlBody>,
) -> (StatusCode, String) {
    let connection = app_state.get_url_repo().get_connection();
    if let Err(connection) = connection {
        tracing::error!(
            "Failed to connect to redis in add url. Error: {}",
            connection
        );
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to connect to redis".to_string(),
        );
    }
    let mut connection = connection.unwrap();
    let ttl = 6 * 60 * 60; // 6 hours
    let id = nanoid::nanoid!(5);
    let url = body.url;
    tracing::debug!("Adding url: {url} with id: {id}");
    let _: Result<String, redis::RedisError> = connection.set_ex(&id, url, ttl);
    (StatusCode::OK, id)
}

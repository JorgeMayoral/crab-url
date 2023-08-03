use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};
use redis::Commands;

use crate::app::AppState;

#[derive(serde::Deserialize)]
pub struct CheckIdBody {
    id: String,
}

pub async fn check_id(
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<CheckIdBody>,
) -> (StatusCode, String) {
    let connection = app_state.get_url_repo().get_connection();
    if let Err(connection) = connection {
        tracing::error!(
            "Failed to connect to redis in check id. Error: {}",
            connection
        );
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to connect to redis".to_string(),
        );
    }
    let mut connection = connection.unwrap();
    let id = body.id;
    let target_url: Result<String, redis::RedisError> = connection.get(&id);
    if target_url.is_err() {
        tracing::debug!("Url with id: {} not found", id);
        return (StatusCode::NOT_FOUND, "Not Found".to_string());
    }
    let target_url = target_url.unwrap();
    (StatusCode::OK, target_url)
}

use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Redirect,
};
use redis::Commands;

use crate::app::AppState;

pub async fn redirect_to_target(
    Path(url_id): Path<String>,
    State(app_state): State<Arc<AppState>>,
) -> (StatusCode, Redirect) {
    let connection = app_state.get_url_repo().get_connection();
    if let Err(connection) = connection {
        tracing::error!(
            "Failed to connect to redis in redirect with id '{url_id}'. Error: {}",
            connection
        );
        return (StatusCode::INTERNAL_SERVER_ERROR, Redirect::to("/404")); // TODO: use a proper page
    }
    let mut connection = connection.unwrap();
    let target_url: Result<String, redis::RedisError> = connection.get(&url_id);
    if target_url.is_err() {
        tracing::debug!("Url with id: {url_id} not found");
        return (StatusCode::NOT_FOUND, Redirect::to("/404"));
    }
    let target_url = target_url.unwrap();
    (StatusCode::SEE_OTHER, Redirect::to(&target_url))
}

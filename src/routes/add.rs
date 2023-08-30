use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};

use crate::{app::AppState, models::UrlRecord};

#[derive(serde::Deserialize)]
pub struct AddUrlBody {
    url: String,
}

#[derive(serde::Serialize)]
pub struct AddUrlResponse {
    data: Option<UrlRecord>,
    error: Option<String>,
}

pub async fn add_url(
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<AddUrlBody>,
) -> (StatusCode, Json<AddUrlResponse>) {
    let id = nanoid::nanoid!(6);
    let url = body.url;
    let ttl = 3 * 60 * 60; // 3 hours in seconds
    let add_url_result = app_state.get_url_repo().add_url(&id, &url, ttl);
    match add_url_result {
        Ok(_) => (),
        Err(add_url_err) => {
            tracing::error!("Failed to add url. Error: {}", add_url_err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(AddUrlResponse {
                    data: None,
                    error: Some("Failed to add url".to_string()),
                }),
            );
        }
    }
    let url_record = UrlRecord::new(id, url, ttl);
    let body = AddUrlResponse {
        data: Some(url_record),
        error: None,
    };
    (StatusCode::CREATED, Json(body))
}

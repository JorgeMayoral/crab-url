use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};

use crate::{app::AppState, models::UrlRecord};

#[derive(serde::Deserialize)]
pub struct CheckIdBody {
    id: String,
}

#[derive(serde::Serialize)]
pub struct CheckUrlResponse {
    data: Option<UrlRecord>,
    error: Option<String>,
}

pub async fn check_id(
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<CheckIdBody>,
) -> (StatusCode, Json<CheckUrlResponse>) {
    let id = body.id;
    let url_repo = app_state.get_url_repo();
    let get_url_result = url_repo.get_url(&id);
    if let Err(get_url_error) = get_url_result {
        tracing::error!("Failed to get url. Error: {}", get_url_error);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(CheckUrlResponse {
                data: None,
                error: Some("Failed to get url".to_string()),
            }),
        );
    }
    let get_ttl_result = url_repo.get_ttl(&id);
    if let Err(get_ttl_error) = get_ttl_result {
        tracing::error!("Failed to get ttl. Error: {}", get_ttl_error);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(CheckUrlResponse {
                data: None,
                error: Some("Failed to get ttl".to_string()),
            }),
        );
    }
    let body = UrlRecord::new(id, get_url_result.unwrap(), get_ttl_result.unwrap());
    let response = CheckUrlResponse {
        data: Some(body),
        error: None,
    };
    (StatusCode::OK, Json(response))
}

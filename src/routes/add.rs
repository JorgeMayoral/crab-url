use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{app::AppState, error::Result, models::UrlRecord};

#[derive(serde::Deserialize)]
pub struct AddUrlBody {
    url: String,
}

#[derive(serde::Serialize)]
pub struct AddUrlResponse {
    data: Option<UrlRecord>,
}

#[tracing::instrument(skip_all)]
pub async fn add_url(
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<AddUrlBody>,
) -> Result<impl IntoResponse> {
    let id = nanoid::nanoid!(6);
    let url = body.url;
    let ttl = 3 * 60 * 60; // 3 hours in seconds
    app_state.get_url_repo().add_url(&id, &url, ttl)?;
    let url_record = UrlRecord::new(id, url, ttl);
    let body = AddUrlResponse {
        data: Some(url_record),
    };
    Ok((StatusCode::CREATED, Json(body)))
}

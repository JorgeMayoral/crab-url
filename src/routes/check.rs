use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{app::AppState, error::Result, models::UrlRecord};

#[derive(serde::Deserialize)]
pub struct CheckIdBody {
    id: String,
}

#[derive(serde::Serialize)]
pub struct CheckUrlResponse {
    data: Option<UrlRecord>,
}

#[tracing::instrument(skip_all)]
pub async fn check_id(
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<CheckIdBody>,
) -> Result<impl IntoResponse> {
    let id = body.id;
    let url_repo = app_state.get_url_repo();
    let url = url_repo.get_url(&id)?;
    let ttl = url_repo.get_ttl(&id)?;
    let body = UrlRecord::new(id, url, ttl);
    let response = CheckUrlResponse { data: Some(body) };
    Ok((StatusCode::OK, Json(response)))
}

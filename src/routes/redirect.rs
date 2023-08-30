use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
};

use crate::{app::AppState, error::Result};

#[tracing::instrument(skip_all, fields(id = %url_id))]
pub async fn redirect_to_target(
    Path(url_id): Path<String>,
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse> {
    let url_repo = app_state.get_url_repo();
    let target_url = url_repo.get_url(&url_id)?;
    Ok((StatusCode::SEE_OTHER, Redirect::to(&target_url)))
}

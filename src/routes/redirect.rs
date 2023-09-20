use std::{net::SocketAddr, sync::Arc};

use axum::{
    extract::{ConnectInfo, Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
};

use crate::{app::AppState, error::Result, services::url_metrics::UrlMetricsService};

#[tracing::instrument(skip_all, fields(id = %url_id))]
pub async fn redirect_to_target(
    Path(url_id): Path<String>,
    ConnectInfo(info): ConnectInfo<SocketAddr>,
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse> {
    let url_repo = app_state.get_url_repo();
    let target_url = url_repo.get_url(&url_id)?;
    tokio::spawn(async move {
        UrlMetricsService::add_visit(&url_id, info).await;
    });
    Ok((StatusCode::SEE_OTHER, Redirect::to(&target_url)))
}

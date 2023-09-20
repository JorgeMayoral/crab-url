use axum::{extract::Query, http::StatusCode, response::IntoResponse};

use crate::services::url_metrics::UrlMetricsService;

#[derive(serde::Deserialize)]
pub struct Params {
    pub url_id: String,
}

#[tracing::instrument(skip_all, fields(id = %params.url_id))]
pub async fn get_metrics(Query(params): Query<Params>) -> impl IntoResponse {
    let url_metrics = UrlMetricsService::get_url_stats(&params.url_id).await;
    (StatusCode::OK, url_metrics.to_string())
}

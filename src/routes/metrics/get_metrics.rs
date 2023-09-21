use axum::{extract::Query, http::StatusCode, Json};

use crate::{models::Metric, services::url_metrics::UrlMetricsService};

#[derive(serde::Deserialize)]
pub struct Params {
    pub url_id: String,
}

#[derive(serde::Serialize)]
pub struct MetricsResponse {
    metrics: Vec<Metric>,
}

#[tracing::instrument(skip_all, fields(id = %params.url_id))]
pub async fn get_metrics(Query(params): Query<Params>) -> (StatusCode, Json<MetricsResponse>) {
    let url_metrics = UrlMetricsService::get_url_stats(&params.url_id).await;
    let body = MetricsResponse {
        metrics: url_metrics,
    };
    (StatusCode::OK, Json(body))
}

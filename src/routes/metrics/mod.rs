use axum::{routing::get, Router};

mod get_metrics;

pub fn generate_metrics_router() -> Router {
    Router::new().route("/", get(get_metrics::get_metrics))
}

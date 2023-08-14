use std::{net::SocketAddr, sync::Arc};

use axum::{
    routing::{get, post},
    Router, Server,
};

mod app;
mod models;
mod routes;
mod telemetry;
mod url_repository;

use app::AppState;
use routes::{add_url, check_id, health_check, not_found, redirect_to_target};
use telemetry::{get_trace_layer, init_tracing_subscriber};
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    init_tracing_subscriber();
    let trace_layer = get_trace_layer();

    let app_state = AppState::new();
    let app_state = Arc::new(app_state);
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/404", get(not_found))
        .route("/add", post(add_url))
        .route("/check", post(check_id))
        .route("/:url_id", get(redirect_to_target))
        .layer(trace_layer)
        .layer(CorsLayer::permissive()) // FIXME: use better CORS policy
        .with_state(app_state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Listening on http://{addr}");
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
}

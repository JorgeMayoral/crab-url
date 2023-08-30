use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router, Server,
};

mod app;
mod cli;
mod models;
mod routes;
mod url_repository;

use app::AppState;
use clap::Parser;
use routes::{add_url, check_id, health_check, not_found, redirect_to_target};
use tower_http::{cors::CorsLayer, trace::TraceLayer};

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    let cli = cli::Cli::parse();
    cli.instrumentation.setup()?;

    let trace_layer = TraceLayer::new_for_http();

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

    let addr = cli.bind;
    tracing::info!(event = "server_start", "Listening on http://{addr}");
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");

    Ok(())
}

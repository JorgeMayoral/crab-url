use std::{io::IsTerminal, net::SocketAddr, sync::Arc};

use axum::{
    routing::{get, post},
    Router, Server,
};

mod app;
mod cli;
mod error;
mod models;
mod routes;
mod trace_layer;
mod url_repository;

use app::AppState;
use clap::Parser;
use routes::{add_url, check_id, health_check, not_found, redirect_to_target};
use tower_http::{services::ServeDir, trace::TraceLayer};

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::config::HookBuilder::default()
        .theme(if !std::io::stderr().is_terminal() {
            color_eyre::config::Theme::new()
        } else {
            color_eyre::config::Theme::dark()
        })
        .install()?;

    let cli = cli::Cli::parse();
    cli.instrumentation.setup()?;

    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(trace_layer::trace_layer_make_span_with)
        .on_request(trace_layer::trace_layer_on_request)
        .on_response(trace_layer::trace_layer_on_response);

    let web_service = ServeDir::new("./frontend/dist");

    let app_state = AppState::new();
    let app_state = Arc::new(app_state);
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/404", get(not_found))
        .route("/add", post(add_url))
        .route("/check", post(check_id))
        .route("/r/:url_id", get(redirect_to_target))
        .fallback_service(web_service)
        .layer(trace_layer)
        .with_state(app_state);

    let addr = cli.bind;
    tracing::info!(event = "server_start", "Listening on http://{addr}");
    Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .expect("Failed to start server");

    Ok(())
}

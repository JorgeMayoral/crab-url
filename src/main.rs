use std::net::SocketAddr;

use axum::{extract::Path, response::Redirect, routing::get, Router, Server};
use tower_http::{
    trace::{DefaultOnResponse, TraceLayer},
    LatencyUnit,
};
use tracing::Level;
use tracing_subscriber::{filter, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    let tracing_filter = filter::Targets::new()
        .with_target("hyper", Level::INFO)
        .with_default(Level::DEBUG);
    tracing_subscriber::registry()
        .with(tracing_filter)
        .with(tracing_logfmt::layer())
        .init();

    let trace_layer = TraceLayer::new_for_http().on_response(
        DefaultOnResponse::new()
            .level(Level::INFO)
            .latency_unit(LatencyUnit::Micros),
    );

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/404", get(not_found))
        .route("/r/:url_id", get(redirect_to_target))
        .layer(trace_layer);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Listening on http://{addr}");
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
}

async fn health_check() -> &'static str {
    "OK"
}

async fn redirect_to_target(Path(url_id): Path<String>) -> Redirect {
    println!("Redirecting to url with id: {url_id}");
    tracing::debug!("Redirecting to url with id: {url_id}");

    if url_id == "not-found" {
        tracing::debug!("Url with id: {url_id} not found");
        return Redirect::to("/404");
    }

    Redirect::to("https://yorch.dev")
}

async fn not_found() -> &'static str {
    "Not Found"
}

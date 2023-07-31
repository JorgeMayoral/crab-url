use std::net::SocketAddr;

use axum::{extract::Path, response::Redirect, routing::get, Router, Server};
use tower_http::trace::TraceLayer;
use tracing::Level;
use tracing_subscriber::{filter, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    let tracing_layer = tracing_subscriber::fmt::layer();
    let tracing_filter = filter::Targets::new()
        .with_target("hyper", Level::INFO)
        .with_default(Level::DEBUG);
    tracing_subscriber::registry()
        .with(tracing_layer)
        .with(tracing_filter)
        .init();

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/404", get(not_found))
        .route("/r/:url_id", get(redirect_to_target))
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
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

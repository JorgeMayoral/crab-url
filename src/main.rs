use std::{net::SocketAddr, sync::Arc};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Redirect,
    routing::{get, post},
    Router, Server,
};
use redis::{Client, Commands};
use tower_http::{
    trace::{DefaultOnResponse, TraceLayer},
    LatencyUnit,
};
use tracing::Level;
use tracing_subscriber::{filter, layer::SubscriberExt, util::SubscriberInitExt};

struct AppState {
    redis_client: Client,
}

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

    dotenvy::dotenv().expect("Failed to load .env file"); // TODO: Find a better to do this
    let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL env var not set");
    let redis_client = Client::open(redis_url).expect("Failed to connect to redis");
    let app_state = Arc::new(AppState { redis_client });
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/404", get(not_found))
        .route("/r/:url_id", get(redirect_to_target))
        .route("/add", post(add_url))
        .layer(trace_layer)
        .with_state(app_state);

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

async fn redirect_to_target(
    Path(url_id): Path<String>,
    State(app_state): State<Arc<AppState>>,
) -> (StatusCode, Redirect) {
    let connection = app_state.redis_client.get_connection();
    if let Err(connection) = connection {
        tracing::error!(
            "Failed to connect to redis in redirect with id '{url_id}'. Error: {}",
            connection
        );
        return (StatusCode::INTERNAL_SERVER_ERROR, Redirect::to("/404")); // TODO: use a proper page
    }
    let mut connection = connection.unwrap();
    let target_url: Result<String, redis::RedisError> = connection.get(&url_id);
    if target_url.is_err() {
        tracing::debug!("Url with id: {url_id} not found");
        return (StatusCode::NOT_FOUND, Redirect::to("/404"));
    }
    let target_url = target_url.unwrap();
    (StatusCode::SEE_OTHER, Redirect::to(&target_url))
}

async fn add_url(State(app_state): State<Arc<AppState>>) -> StatusCode {
    let connection = app_state.redis_client.get_connection();
    if let Err(connection) = connection {
        tracing::error!(
            "Failed to connect to redis in add url. Error: {}",
            connection
        );
        return StatusCode::INTERNAL_SERVER_ERROR;
    }
    let mut connection = connection.unwrap();
    let ttl = 24 * 60 * 60; // 24 hours
    let _: Result<String, redis::RedisError> = connection.set_ex("test", "https://yorch.dev", ttl);
    StatusCode::OK
}

async fn not_found() -> &'static str {
    "Not Found"
}

use std::net::SocketAddr;

use axum::{extract::Path, response::Redirect, routing::get, Router, Server};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/r/:url_id", get(redirect_to_target))
        .route("/404", get(not_found));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{addr}");
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

    if url_id == "not-found" {
        return Redirect::to("/404");
    }

    Redirect::to("https://yorch.dev")
}

async fn not_found() -> &'static str {
    "Not Found"
}

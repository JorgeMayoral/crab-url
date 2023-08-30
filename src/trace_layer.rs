use axum::{body::BoxBody, extract::ConnectInfo, response::Response};
use hyper::{Body, Request};
use std::{net::SocketAddr, time::Duration};
use tracing::Span;

pub fn trace_layer_make_span_with(request: &Request<Body>) -> Span {
    tracing::error_span!("request",
      uri = %request.uri(),
      method = %request.method(),
      source = request.extensions()
        .get::<ConnectInfo<SocketAddr>>()
        .map(|connect_info|
          tracing::field::display(connect_info.ip().to_string()),
        ).unwrap_or_else(||
          tracing::field::display(String::from("<unknown>"))
        ),
      status = tracing::field::Empty,
      latency = tracing::field::Empty,
    )
}

pub fn trace_layer_on_request(_request: &Request<Body>, _span: &Span) {
    tracing::info!(event = "request_received")
}

pub fn trace_layer_on_response(response: &Response<BoxBody>, latency: Duration, span: &Span) {
    span.record(
        "latency",
        tracing::field::display(format!("{}μs", latency.as_micros())),
    );
    span.record("status", tracing::field::display(response.status()));
    tracing::info!(event = "request_completed")
}

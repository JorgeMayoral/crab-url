use tower_http::{
    classify::{ServerErrorsAsFailures, SharedClassifier},
    trace::{DefaultOnResponse, TraceLayer},
    LatencyUnit,
};
use tracing::Level;
use tracing_subscriber::{
    filter::{self, Targets},
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

pub fn init_tracing_subscriber() {
    let tracing_filter = get_tracing_filter();
    tracing_subscriber::registry()
        .with(tracing_filter)
        .with(tracing_logfmt::layer())
        .init();
}

pub fn get_trace_layer() -> TraceLayer<SharedClassifier<ServerErrorsAsFailures>> {
    TraceLayer::new_for_http().on_response(
        DefaultOnResponse::new()
            .level(Level::INFO)
            .latency_unit(LatencyUnit::Millis),
    )
}

fn get_tracing_filter() -> Targets {
    filter::Targets::new()
        .with_target("hyper", Level::INFO)
        .with_default(Level::DEBUG)
}

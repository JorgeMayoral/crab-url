#[tracing::instrument(skip_all)]
pub async fn health_check() -> &'static str {
    "OK"
}

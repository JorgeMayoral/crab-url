#[tracing::instrument(skip_all)]
pub async fn not_found() -> &'static str {
    "Not Found"
}

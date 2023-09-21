#[derive(serde::Serialize)]
pub struct Metric {
    pub url_id: String,
    pub country: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl Metric {
    pub fn from_row(row: &libsql_client::Row) -> Self {
        let url_id: &str = row.try_column("url").unwrap();
        let country: &str = row.try_column("country").unwrap();
        let timestamp: &str = row.try_column("timestamp").unwrap();
        Self {
            url_id: url_id.to_owned(),
            country: country.to_owned(),
            timestamp: chrono::DateTime::parse_from_rfc3339(timestamp)
                .unwrap()
                .with_timezone(&chrono::Utc),
        }
    }
}

use libsql_client::{args, Statement};

use crate::models::Metric;

pub struct MetricsClient {
    db: libsql_client::Client,
}

impl MetricsClient {
    pub async fn new() -> Self {
        let db = libsql_client::Client::from_env().await.unwrap();
        Self { db }
    }

    pub async fn add_metric(
        &self,
        url: String,
        country: String,
        timestamp: String,
    ) -> Result<(), anyhow::Error> {
        let _ = self
            .db
            .execute(Statement::with_args(
                "INSERT INTO metrics (url, country, timestamp) VALUES (?, ?, ?)",
                args!(url, country, timestamp),
            ))
            .await?;

        Ok(())
    }

    pub async fn get_url_metrics(&self, url: String) -> Result<Vec<Metric>, anyhow::Error> {
        let res = self
            .db
            .execute(Statement::with_args(
                "SELECT * FROM metrics WHERE url = ?",
                args!(url),
            ))
            .await?;
        let metrics: Vec<Metric> = res.rows.iter().map(Metric::from_row).collect();
        Ok(metrics)
    }
}

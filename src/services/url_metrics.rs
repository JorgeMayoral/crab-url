use std::net::SocketAddr;

use ipgeolocate::{Locator, Service};

use crate::models::Metric;

use super::metrics_client::MetricsClient;

pub struct UrlMetricsService {}

impl UrlMetricsService {
    pub async fn add_visit(url_id: &str, addr: SocketAddr) {
        let ip = addr.ip();
        let geolocation_service = Service::IpApi;
        let metrics_client = MetricsClient::new().await;
        let timestamp = chrono::Utc::now();
        let country = match Locator::get_ipaddr(ip, geolocation_service).await {
            Ok(geolocation) => {
                let country = geolocation.country;
                tracing::info!(
                    "Adding visit for \"{url_id}\" from {ip} in {country} at {timestamp}"
                );
                country
            }
            Err(e) => {
                tracing::error!("Failed to get geolocation for {ip}: {e}");
                "unknown".to_owned()
            }
        };
        if let Err(e) = metrics_client
            .add_metric(url_id.to_owned(), country, timestamp.to_rfc3339())
            .await
        {
            tracing::error!("Failed to add metric for {ip}. Error: {e}");
        }
    }

    pub async fn get_url_stats(url_id: &str) -> Vec<Metric> {
        tracing::info!("Getting stats for \"{url_id}\"");
        let metrics_client = MetricsClient::new().await;
        match metrics_client.get_url_metrics(url_id.to_owned()).await {
            Ok(metrics) => metrics,
            Err(e) => {
                tracing::error!("Failed to get metrics for {url_id}. Error: {e}");
                vec![]
            }
        }
    }
}

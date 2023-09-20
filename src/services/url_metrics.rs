use std::net::SocketAddr;

use ipgeolocate::{Locator, Service};

pub struct UrlMetricsService {}

impl UrlMetricsService {
    pub async fn add_visit(url_id: &str, addr: SocketAddr) {
        let ip = addr.ip();
        let geolocation_service = Service::IpApi;

        match Locator::get_ipaddr(ip, geolocation_service).await {
            Ok(geolocation) => {
                let Locator {
                    ip,
                    latitude: _,
                    longitude: _,
                    city: _,
                    region: _,
                    country,
                    timezone: _,
                } = geolocation;

                let timestamp = chrono::Utc::now();
                tracing::info!(
                    "Adding visit for \"{url_id}\" from {ip} in {country} at {timestamp}"
                );
            }
            Err(e) => {
                tracing::error!("Failed to get geolocation for {ip}: {e}");
            }
        };
    }

    pub async fn get_url_stats(url_id: &str) -> u64 {
        tracing::info!("Getting stats for \"{url_id}\"");
        0
    }
}

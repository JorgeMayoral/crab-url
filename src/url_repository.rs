use redis::{Client, Commands};

use crate::error::{AppError, Result};

pub struct UrlRepository {
    client: Client,
}

impl UrlRepository {
    pub fn new() -> Self {
        dotenvy::dotenv().expect("Failed to load .env file"); // TODO: Find a better way to do this
        let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL env var not set");
        let redis_client = Client::open(redis_url).expect("Failed to connect to redis");
        Self {
            client: redis_client,
        }
    }

    pub fn get_connection(&self) -> Result<redis::Connection, AppError> {
        match self.client.get_connection() {
            Ok(connection) => Ok(connection),
            Err(err) => {
                tracing::error!("Failed to get redis connection. Error: {}", err);
                Err(AppError::UrlRepository(err.to_string()))
            }
        }
    }

    pub fn get_url(&self, id: &str) -> Result<String, AppError> {
        let mut connection = self.get_connection()?;
        tracing::debug!("Getting url with id: {}", id);
        match connection.get(id) {
            Ok(url) => Ok(url),
            Err(err) => {
                tracing::error!("Failed to get url from redis. Error: {}", err);
                Err(AppError::UrlRepository(err.to_string()))
            }
        }
    }

    pub fn get_ttl(&self, id: &str) -> Result<usize, AppError> {
        let mut connection = self.get_connection()?;
        tracing::debug!("Getting ttl for url with id: {}", id);
        match connection.ttl(id) {
            Ok(ttl) => Ok(ttl),
            Err(err) => {
                tracing::error!("Failed to get ttl from redis. Error: {}", err);
                Err(AppError::UrlRepository(err.to_string()))
            }
        }
    }

    pub fn add_url(&self, id: &str, target: &str, ttl: usize) -> Result<(), AppError> {
        let mut connection = self.get_connection()?;
        tracing::debug!("Adding url: {target} with id: {id}");
        let add_url_result: Result<(), redis::RedisError> = connection.set_ex(id, target, ttl);
        match add_url_result {
            Ok(_) => Ok(()),
            Err(err) => {
                tracing::error!("Failed to add url to redis. Error: {}", err);
                Err(AppError::UrlRepository(err.to_string()))
            }
        }
    }
}

use redis::{Client, Commands};

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

    pub fn get_connection(&self) -> Result<redis::Connection, redis::RedisError> {
        self.client.get_connection()
    }

    pub fn get_url(&self, id: &str) -> Result<String, redis::RedisError> {
        let connection = self.get_connection();
        if let Err(conn_error) = connection {
            tracing::error!(
                "Failed to connect to redis in get url. Error: {}",
                conn_error
            );
            return Err(conn_error);
        }
        let mut connection = connection.unwrap();
        tracing::debug!("Getting url with id: {}", id);
        connection.get(id)
    }

    pub fn get_ttl(&self, id: &str) -> Result<usize, redis::RedisError> {
        let connection = self.get_connection();
        if let Err(conn_error) = connection {
            tracing::error!(
                "Failed to connect to redis in get ttl. Error: {}",
                conn_error
            );
            return Err(conn_error);
        }
        let mut connection = connection.unwrap();
        tracing::debug!("Getting ttl for url with id: {}", id);
        connection.ttl(id)
    }

    pub fn add_url(&self, id: &str, target: &str, ttl: usize) -> Result<(), redis::RedisError> {
        let connection = self.get_connection();
        if let Err(conn_error) = connection {
            tracing::error!(
                "Failed to connect to redis in add url. Error: {}",
                conn_error
            );
            return Err(conn_error);
        }
        let mut connection = connection.unwrap();
        tracing::debug!("Adding url: {target} with id: {id}");
        connection.set_ex(id, target, ttl)
    }
}

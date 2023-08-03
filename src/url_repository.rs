use redis::Client;

pub struct UrlRepository {
    client: Client,
}

impl UrlRepository {
    pub fn new() -> Self {
        dotenvy::dotenv().expect("Failed to load .env file"); // TODO: Find a better to do this
        let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL env var not set");
        let redis_client = Client::open(redis_url).expect("Failed to connect to redis");
        Self {
            client: redis_client,
        }
    }

    pub fn get_connection(&self) -> Result<redis::Connection, redis::RedisError> {
        self.client.get_connection()
    }
}

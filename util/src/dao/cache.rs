use crate::{ApiError, CacheConfig};
use redis::Client;

#[derive(Debug)]
pub struct Cache {
    client: Client,
}

impl Cache {
    pub async fn new(cfg: CacheConfig) -> Result<Self, ApiError> {
        let cache_url = format!("redis://:{}@{}:{}/", cfg.password, cfg.host, cfg.port);
        let client = redis::Client::open(cache_url)?;
        Ok(Cache {
            client,
        })
    }

    pub fn get_client(&self) -> &Client {
        &self.client
    }
}

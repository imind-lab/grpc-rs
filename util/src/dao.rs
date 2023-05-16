pub mod cache;
pub mod database;

use crate::{ApiError, Config};
use cache::Cache;
use database::Database;
use redis::Client;
use sqlx::{MySql, Pool};

pub trait DataOperator {
    fn get_database_pool(&self) -> &Pool<MySql>;
    fn get_cache_client(&self) -> &Client;
}

#[derive(Debug, Clone)]
pub struct Dao {
    database: Database,
    cache: Cache,
}

impl Dao {
    pub async fn new(cfg: Config) -> Result<Self, ApiError> {
        let database = Database::new(cfg.db).await?;
        let cache = Cache::new(cfg.cache).await?;
        Ok(Self { database, cache })
    }
}

impl DataOperator for Dao {
    fn get_database_pool(&self) -> &Pool<MySql> {
        self.database.get_pool()
    }

    fn get_cache_client(&self) -> &Client {
        self.cache.get_client()
    }
}

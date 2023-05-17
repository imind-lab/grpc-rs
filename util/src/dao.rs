pub mod cache;
pub mod database;

use crate::{ApiError, CacheConfig, DBConfig};
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
    pub async fn new(db: DBConfig, cache: CacheConfig) -> Result<Self, ApiError> {
        let database = Database::new(db).await?;
        let cache = Cache::new(cache).await?;
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

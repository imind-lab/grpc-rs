use crate::{ApiError, DBConfig};
use sqlx::{
    mysql::{MySql, MySqlPool},
    Pool,
};

#[derive(Debug, Clone)]
pub struct Database {
    pool: Pool<MySql>,
}

impl Database {
    pub async fn new(cfg: DBConfig) -> Result<Self, ApiError> {
        let db_url = format!(
            "mysql://{}:{}@{}:{}/{}",
            cfg.user, cfg.password, cfg.host, cfg.port, cfg.dbname
        );
        let pool = MySqlPool::connect(&db_url)
            .await
            .map_err(ApiError::SqlxError)?;
        Ok(Database { pool })
    }

    pub fn get_pool(&self) -> &Pool<MySql> {
        &self.pool
    }
}

use tonic::async_trait;
use util::{Dao, ApiError, DataOperator, get_timestamp};
use crate::application::{CreateMicro, Micro};

#[async_trait]
pub trait MicroRepository {
    async fn find(&self, dao: &Dao, id: u32) -> Result<Micro, ApiError>;
    async fn create(&self, dao: &Dao, model: CreateMicro) -> Result<Micro, ApiError>; 
}

pub struct MicroStore;

#[async_trait]
impl MicroRepository for MicroStore {
    async fn find(&self, dao: &Dao, id: u32) -> Result<Micro, ApiError> {
        let mut conn = dao.get_cache_client().get_async_connection().await?;
        let key = format!("micro_{}", id);
        let ret: Result<Micro, redis::RedisError> = redis::Cmd::hgetall(key.clone())
            .query_async(&mut conn)
            .await;
        if let Ok(micro) = ret {
            return Ok(micro);
        }

        let micro = sqlx::query_as::<_, Micro>("SELECT * FROM micro WHERE id=?")
            .bind(id)
            .fetch_one(dao.get_database_pool())
            .await?;

        let _: Result<(), redis::RedisError> = redis::cmd("HSET")
            .arg(key)
            .arg(&micro)
            .query_async(&mut conn)
            .await;

        Ok(micro)
    }

    async fn create(&self, dao: &Dao, model: CreateMicro) -> Result<Micro, ApiError> {
        let now = get_timestamp();
        println!("now: {}", now);
        let mut transaction = dao.get_database_pool().begin().await?;
        let id = sqlx::query(
            "INSERT INTO micro (name, typ, update_ts, create_ts) VALUES (?, ?, ?, ?)",
        )
        .bind(model.name)
        .bind(model.typ)
        .bind(now)
        .bind(now)
        .execute(&mut transaction)
        .await?
        .last_insert_id();

        let micro = sqlx::query_as::<_, Micro>("SELECT * FROM micro WHERE id=?")
            .bind(id)
            .fetch_one(&mut transaction)
            .await?;

        transaction.commit().await?;

        let mut conn = dao.get_cache_client().get_async_connection().await?;
        let key = format!("micro_{}", id);
        let _: Result<(), redis::RedisError> = redis::cmd("HSET")
            .arg(key)
            .arg(&micro)
            .query_async(&mut conn)
            .await;

        Ok(micro)
    }
}
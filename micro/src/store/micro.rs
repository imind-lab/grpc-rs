use std::sync::Arc;

use futures::TryStreamExt;

use crate::application::{micro::Micro as ProtoMicro, CreateMicro, Micro};
use tonic::async_trait;
use util::{get_timestamp, ApiError, Dao, DataOperator};

#[async_trait]
pub trait MicroRepository {
    async fn find(&self, id: u32) -> Result<Micro, ApiError>;
    async fn create(&self, model: CreateMicro) -> Result<Micro, ApiError>;
    async fn list(
        &self,
        typ: i8,
        page_size: u16,
        page_num: u16,
        is_desc: bool,
    ) -> Result<Vec<ProtoMicro>, ApiError>;
}

use tokio::task;

#[derive(Debug, Clone)]
pub struct MicroStore {
    dao: Arc<Dao>,
}

impl MicroStore {
    pub fn new(dao: Dao) -> Self {
        Self { dao: Arc::new(dao) }
    }
}

#[async_trait]
impl MicroRepository for MicroStore {
    async fn find(&self, id: u32) -> Result<Micro, ApiError> {
        let mut conn = self.dao.get_cache_client().get_async_connection().await?;
        let key = format!("micro_{}", id);
        let ret: Result<Micro, redis::RedisError> = redis::Cmd::hgetall(key.clone())
            .query_async(&mut conn)
            .await;
        if let Ok(micro) = ret {
            return Ok(micro);
        }

        let micro = sqlx::query_as::<_, Micro>("SELECT * FROM micro WHERE id=?")
            .bind(id)
            .fetch_one(self.dao.get_database_pool())
            .await?;

        let _: Result<(), redis::RedisError> = redis::cmd("HSET")
            .arg(key)
            .arg(&micro)
            .query_async(&mut conn)
            .await;

        Ok(micro)
    }

    async fn create(&self, model: CreateMicro) -> Result<Micro, ApiError> {
        let now = get_timestamp();
        println!("now: {}", now);
        let mut transaction = self.dao.get_database_pool().begin().await?;
        let id =
            sqlx::query("INSERT INTO micro (name, typ, update_ts, create_ts) VALUES (?, ?, ?, ?)")
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

        let mut conn = self.dao.get_cache_client().get_async_connection().await?;
        let key = format!("micro_{}", id);
        let _: Result<(), redis::RedisError> = redis::cmd("HSET")
            .arg(key)
            .arg(&micro)
            .query_async(&mut conn)
            .await;

        Ok(micro)
    }

    async fn list(
        &self,
        typ: i8,
        page_size: u16,
        page_num: u16,
        is_desc: bool,
    ) -> Result<Vec<ProtoMicro>, ApiError> {
        let mut conn = self.dao.get_cache_client().get_async_connection().await?;
        let key = format!("micros_typ_{}_{}_{}", typ, page_num, is_desc);
        let exist: bool = redis::Cmd::exists(key.clone())
            .query_async(&mut conn)
            .await?;
        let start = ((page_num - 1) * page_size) as isize;
        let end = start + page_size as isize;
        if exist {
            let ret: Result<Vec<u32>, redis::RedisError> =
                redis::Cmd::zrange(key.clone(), start, end - 1)
                    .query_async(&mut conn)
                    .await;
            if let Ok(ids) = ret {
                let mut handles = Vec::with_capacity(ids.len());
                for &id in ids.iter() {
                    let store = self.clone();
                    let join = task::spawn(async move { store.find(id).await });
                    handles.push(join);
                }
                let mut micros = Vec::with_capacity(ids.len());
                for handle in handles {
                    if let Ok(Ok(micro)) = handle.await {
                        micros.push(micro.into())
                    }
                }
                if micros.len() == ids.len() {
                    return Ok(micros);
                }
            }
        }

        let pool = self.dao.get_database_pool();
        let mut stream = sqlx::query_as::<_, Micro>("SELECT * FROM micro WHERE typ=?")
            .bind(typ)
            .fetch(pool);

        let mut micros = Vec::new();
        let mut args: Vec<i64> = Vec::new();
        let mut pipeline = &mut redis::pipe();
        let mut i: isize = 0;
        while let Some(micro) = stream.try_next().await? {
            if i >= start && i < end {
                micros.push(micro.clone().into());
            }
            pipeline = pipeline
                .cmd("HMSET")
                .arg(format!("micro_{}", micro.id))
                .arg(&micro)
                .ignore();
            args.push(micro.update_ts);
            args.push(micro.id as _);
            i += 1;
        }

        let _: Result<(), redis::RedisError> = pipeline
            .cmd("ZADD")
            .arg(key)
            .arg(&args)
            .query_async(&mut conn)
            .await;

        println!("micros: {:?}", micros);

        Ok(micros)
    }
}

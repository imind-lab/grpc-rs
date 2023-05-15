use tonic::async_trait;

use util::{Dao, ApiError};
use crate::application::{CreateMicro, Micro};
use crate::store::{MicroStore, MicroRepository};

#[async_trait]
pub trait MicroBusiness {
    async fn find(&self, dao: &Dao, id: u32) -> Result<Micro, ApiError>;
    async fn create(&self, dao: &Dao, model: CreateMicro) -> Result<Micro, ApiError>; 
}

pub struct MicroDomain;

#[async_trait]
impl MicroBusiness for MicroDomain {
    async fn find(&self, dao: &Dao, id: u32) -> Result<Micro, ApiError> {
        let micro_store = MicroStore;
        micro_store.find(dao, id).await
    }

    async fn create(&self, dao: &Dao, model: CreateMicro) -> Result<Micro, ApiError> {
        let micro_store = MicroStore;
        micro_store.create(dao, model).await
    }
}
use tonic::async_trait;

use crate::application::{micro::Micro as ProtoMicro, CreateMicro, Micro};
use crate::store::{MicroRepository, MicroStore};
use util::{ApiError, Dao};

#[async_trait]
pub trait MicroBusiness {
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

#[derive(Debug, Clone)]
pub struct MicroDomain {
    store: MicroStore,
}

impl MicroDomain {
    pub fn new(dao: Dao) -> Self {
        let store = MicroStore::new(dao);
        Self { store }
    }
}

#[async_trait]
impl MicroBusiness for MicroDomain {
    async fn find(&self, id: u32) -> Result<Micro, ApiError> {
        self.store.find(id).await
    }

    async fn create(&self, model: CreateMicro) -> Result<Micro, ApiError> {
        self.store.create(model).await
    }

    async fn list(
        &self,
        typ: i8,
        page_size: u16,
        page_num: u16,
        is_desc: bool,
    ) -> Result<Vec<ProtoMicro>, ApiError> {
        self.store.list(typ, page_size, page_num, is_desc).await
    }
}

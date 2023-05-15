use tonic::{Request, Response, Status};
use pb::micro_service_server::MicroService as MicroTrait;


pub mod pb {
    tonic::include_proto!("micro");
    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("micro_descriptor");
}

use util::Dao;

pub use pb::{
    Micro,
    micro_service_server::MicroServiceServer,
    GetMicroByIdRequest, GetMicroByIdResponse,
    CreateMicroRequest, CreateMicroResponse,
};

use crate::domain::{MicroDomain, MicroBusiness};

#[derive(Debug)]
pub struct MicroService{
    dao: Dao,
}

impl MicroService {
    pub fn new(dao: Dao) -> Self {
        MicroService {
            dao
        }
    }

    pub fn get_dao(&self) -> &Dao {
        &self.dao
    }
}

#[tonic::async_trait]
impl MicroTrait for MicroService {
    async fn get_micro_by_id(&self, request: Request<GetMicroByIdRequest>) -> Result<Response<GetMicroByIdResponse>, Status> {
        let req = request.into_inner();

        let domain = MicroDomain;
        let result = domain.find(self.get_dao(), req.id).await;

        match result {
            Ok(micro) => {
                println!("{:?}", micro);
                Ok(Response::new(pb::GetMicroByIdResponse {
                    code: 200,
                    msg: "".to_string(),
                    data: Some( micro.into())
                }))
            }
            Err(err) => {
                println!("{:?}", err);
                Err(err.into())
            }
        }
    }

    async fn create_micro(&self, request: Request<CreateMicroRequest>) -> Result<Response<CreateMicroResponse>, Status> {
        let req = request.into_inner();
        let domain = MicroDomain;
        let result = domain.create(self.get_dao(), req.into()).await;

        match result {
            Ok(micro) => {
                println!("{:?}", micro);
                Ok(Response::new(pb::CreateMicroResponse {
                    code: 200,
                    msg: "".to_string(),
                }))
            }
            Err(err) => {
                println!("err{:?}", err);
                Err(err.into())
            }
        }
    }
}

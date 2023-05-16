use pb::micro_service_server::MicroService as MicroTrait;
use tonic::{Request, Response, Status};

pub mod pb {
    tonic::include_proto!("micro");
    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("micro_descriptor");
}

use util::Dao;

pub use pb::{
    micro_service_server::MicroServiceServer, CreateMicroRequest, CreateMicroResponse,
    GetMicroByIdRequest, GetMicroByIdResponse, GetMicroListRequest, GetMicroListResponse, Micro,
    MicroList,
};

use crate::domain::{MicroBusiness, MicroDomain};

#[derive(Debug, Clone)]
pub struct MicroService {
    domain: MicroDomain,
}

impl MicroService {
    pub fn new(dao: Dao) -> Self {
        let domain = MicroDomain::new(dao);
        Self { domain }
    }
}

#[tonic::async_trait]
impl MicroTrait for MicroService {
    async fn get_micro_by_id(
        &self,
        request: Request<GetMicroByIdRequest>,
    ) -> Result<Response<GetMicroByIdResponse>, Status> {
        let req = request.into_inner();

        let result = self.domain.find(req.id).await;

        match result {
            Ok(micro) => {
                println!("{:?}", micro);
                Ok(Response::new(pb::GetMicroByIdResponse {
                    code: 200,
                    msg: "".to_string(),
                    data: Some(micro.into()),
                }))
            }
            Err(err) => {
                println!("{:?}", err);
                Err(err.into())
            }
        }
    }

    async fn create_micro(
        &self,
        request: Request<CreateMicroRequest>,
    ) -> Result<Response<CreateMicroResponse>, Status> {
        let req = request.into_inner();
        let result = self.domain.create(req.into()).await;

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

    async fn get_micro_list(
        &self,
        request: Request<GetMicroListRequest>,
    ) -> Result<Response<GetMicroListResponse>, Status> {
        let req = request.into_inner();
        let typ = req.typ as i8;
        let page_size = req.page_size as u16;
        let page_num = req.page_num as u16;

        let result = self
            .domain
            .list(typ, page_size, page_num, req.is_desc)
            .await;

        println!("result: {:?}", result);

        match result {
            Ok(micros) => Ok(Response::new(pb::GetMicroListResponse {
                code: 200,
                msg: "".to_string(),
                data: Some(MicroList {
                    total: 10,
                    total_page: 2,
                    cur_page: 1,
                    datalist: micros,
                }),
            })),
            Err(err) => Err(err.into()),
        }
    }
}

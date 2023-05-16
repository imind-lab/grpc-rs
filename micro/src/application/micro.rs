use pb::micro_service_server::MicroService as MicroTrait;
use tokio::sync::mpsc;
use tonic::{Request, Response, Status, Streaming};

pub mod pb {
    tonic::include_proto!("micro");
    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("micro_descriptor");
}

use futures::Stream;
use std::{error::Error, io::ErrorKind, pin::Pin};
use tokio_stream::{wrappers::ReceiverStream, StreamExt};
use util::Dao;

pub use pb::{
    micro_service_server::MicroServiceServer, CreateMicroRequest, CreateMicroResponse,
    GetMicroByIdRequest, GetMicroByIdResponse, GetMicroListByStreamRequest,
    GetMicroListByStreamResponse, GetMicroListRequest, GetMicroListResponse, Micro, MicroList,
};

use crate::domain::{MicroBusiness, MicroDomain};

type ResponseStream =
    Pin<Box<dyn Stream<Item = Result<GetMicroListByStreamResponse, Status>> + Send>>;

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

    type GetMicroListByStreamStream = ResponseStream;

    async fn get_micro_list_by_stream(
        &self,
        req: Request<Streaming<GetMicroListByStreamRequest>>,
    ) -> Result<Response<Self::GetMicroListByStreamStream>, Status> {
        let mut in_stream = req.into_inner();

        let (tx, rx) = mpsc::channel(128);

        let domain = self.domain.clone();

        tokio::spawn(async move {
            while let Some(result) = in_stream.next().await {
                match result {
                    Ok(data) => {
                        match domain.find(data.id).await {
                            Ok(micro) => {
                                println!("{:?}", micro);
                                tx.send(Ok(GetMicroListByStreamResponse {
                                    index: data.index,
                                    result: Some(micro.into()),
                                }))
                                .await
                                .expect("working rx");
                            }
                            Err(err) => {
                                match tx.send(Err(err.into())).await {
                                    Ok(_) => (),
                                    Err(_err) => break, // response was droped
                                }
                            }
                        }
                    }
                    Err(err) => {
                        if let Some(io_err) = match_for_io_error(&err) {
                            if io_err.kind() == ErrorKind::BrokenPipe {
                                // here you can handle special case when client
                                // disconnected in unexpected way
                                eprintln!("\tclient disconnected: broken pipe");
                                break;
                            }
                        }

                        match tx.send(Err(err)).await {
                            Ok(_) => (),
                            Err(_err) => break, // response was droped
                        }
                    }
                };
            }
            println!("\tstream ended");
        });

        // echo just write the same data that was received
        let out_stream = ReceiverStream::new(rx);

        Ok(Response::new(
            Box::pin(out_stream) as Self::GetMicroListByStreamStream
        ))
    }
}

fn match_for_io_error(err_status: &Status) -> Option<&std::io::Error> {
    let mut err: &(dyn Error + 'static) = err_status;

    loop {
        if let Some(io_err) = err.downcast_ref::<std::io::Error>() {
            return Some(io_err);
        }

        // h2::Error do not expose std::io::Error with `source()`
        // https://github.com/hyperium/h2/pull/462
        if let Some(h2_err) = err.downcast_ref::<h2::Error>() {
            if let Some(io_err) = h2_err.get_io() {
                return Some(io_err);
            }
        }

        err = match err.source() {
            Some(err) => err,
            None => return None,
        };
    }
}

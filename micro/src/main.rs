mod application;
mod domain;
mod store;

use dotenvy::dotenv;

use application::micro::{MicroService, MicroServiceServer};
use tonic::transport::Server;
use util::{Config, Dao};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "micro=debug");
    }

    tracing_subscriber::fmt::init();

    dotenv().ok();

    let cfg = Config::from_env("micro").expect("初始化配置失败");

    let dao = Dao::new(cfg).await.expect("Dao required");

    let service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(crate::application::micro::pb::FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    let addr = "0.0.0.0:8088".parse().unwrap();

    let micro_service = MicroService::new(dao);

    Server::builder()
        .add_service(service)
        .add_service(MicroServiceServer::new(micro_service))
        .serve(addr)
        .await?;
    Ok(())
}

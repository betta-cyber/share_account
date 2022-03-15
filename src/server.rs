#[macro_use]
extern crate lazy_static;
extern crate log;

use log::{debug, info};
use config::Config;
use tonic::{transport::Server, Request, Response, Status};

use share_account_mod::*;

pub mod share_account_mod {
    tonic::include_proto!("share_account");
}

lazy_static! {
    static ref SETTINGS: Config = Config::builder()
        .add_source(config::File::with_name("Settings"))
        .build()
        .unwrap();

}

#[derive(Default)]
pub struct ShareAccountImpl {}

#[tonic::async_trait]
impl share_account_server::ShareAccount for ShareAccountImpl {
    // rpc for service
    async fn web_request(
        &self,
        _req: Request<Empty>,
    ) -> Result<Response<Data>, Status> {
        // create Response
        let mut phone_grpc = "http://".to_string();
        let phone_grpc_addr = SETTINGS.get_string("phone_grpc_addr").unwrap();
        phone_grpc += &phone_grpc_addr;

        let mut client = share_account_client::ShareAccountClient::connect(phone_grpc).await.unwrap();
        let req = tonic::Request::new(
            Empty {}
        );

        // send the request
        let resp = client.phone_request(req).await;

        let res = match resp {
            Ok(r) => r.into_inner().urlscheme,
            Err(_) => "error".to_owned(),
        };

        let reply = Data {
            urlscheme: res,
        };
        debug!("get request from {:#?}", reply);

        Ok(Response::new(reply))
    }

    async fn phone_request(
        &self,
        _req: Request<Empty>,
    ) -> Result<Response<Data>, Status> {
        // create Response
        // let mut r = Data::new();
        let reply = Data {
            urlscheme: format!("111111 world"),
        };
        let name = " 11111 1world";
        // sent the response
        debug!("get request from {:#?}", name);
        // r.set_urlscheme(format!("Hello {}", name));
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let server_grpc_addr = SETTINGS.get_string("server_grpc_addr").unwrap();
    let addr = server_grpc_addr.parse().unwrap();
    let share_server = ShareAccountImpl::default();

    info!("share account listening on {}", addr);

    Server::builder()
        .add_service(share_account_server::ShareAccountServer::new(share_server))
        .serve(addr)
        .await?;

    Ok(())
}

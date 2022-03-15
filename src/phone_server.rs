use std::env;
use std::thread;
use std::time;

use tonic::{transport::Server, Request, Response, Status};

use share_account_mod::*;

pub mod share_account_mod {
    tonic::include_proto!("share_account");
}

use grpc::ServerRequestSingle;
use grpc::ServerResponseSink;
use grpc::ServerHandlerContext;

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
        // let mut r = Data::new();
        let reply = Data {
            urlscheme: format!("world"),
        };
        let name = "world";
        // sent the response
        println!("greeting request from {}", name);
        // r.set_urlscheme(format!("Hello {}", name));

        // thread::sleep(time::Duration::from_millis(4000));

        Ok(Response::new(reply))
    }

    async fn phone_request(
        &self,
        _req: Request<Empty>,
    ) -> Result<Response<Data>, Status> {
        // create Response
        // let mut r = Data::new();
        let reply = Data {
            urlscheme: format!("111111 xxxxxx"),
        };
        let name = " 11111 1 xxxxx";
        // sent the response
        println!("greeting request from {}", name);
        // r.set_urlscheme(format!("Hello {}", name));
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50052".parse().unwrap();
    let share_server = ShareAccountImpl::default();

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(share_account_server::ShareAccountServer::new(share_server))
        .serve(addr)
        .await?;

    Ok(())
}

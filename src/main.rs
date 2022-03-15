#![deny(warnings)]

#[macro_use]
extern crate lazy_static;
extern crate log;

use config::Config;
use axum::{
    routing::get,
    Router,
};

use share_account_mod::*;
use log::debug;

pub mod share_account_mod {
    tonic::include_proto!("share_account");
}

lazy_static! {
    static ref SETTINGS: Config = Config::builder()
        .add_source(config::File::with_name("Settings"))
        .build()
        .unwrap();

}


#[tokio::main]
async fn main() {
    env_logger::init();
    let server_addr = SETTINGS.get_string("server_addr").unwrap();

    let app = Router::new()
        .route("/", get(get_grpc_result));

    // run it with hyper on localhost:3000
    axum::Server::bind(&server_addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_grpc_result<'x>() -> String {
    let mut server_grpc = "http://".to_string();
    let server_grpc_config = SETTINGS.get_string("server_grpc_addr").unwrap();
    server_grpc += &server_grpc_config;

    let mut client = share_account_client::ShareAccountClient::connect(server_grpc).await.unwrap();
    let req = tonic::Request::new(
        Empty {}
    );

    // send the request
    let resp = client.web_request(req).await;
    let res = match resp {
        Ok(r) => r.into_inner().urlscheme,
        Err(_) => "error".to_owned(),
    };
    debug!("{:#?}", res);
    return res.to_string()
}

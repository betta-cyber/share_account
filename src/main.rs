#![deny(warnings)]

// use std::io;
// use std::convert::Infallible;
// use std::str::FromStr;
// use std::time::Duration;
use axum::{
    routing::get,
    Router,
};
// use tonic::transport::Channel;
// use tonic::Request;

use share_account_mod::*;

// use grpc::ClientStub;
// use grpc::ClientStubExt;
// use futures::executor;

pub mod share_account_mod {
    tonic::include_proto!("share_account");
}


#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(get_grpc_result));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_grpc_result<'x>() -> String {
    // let port = 50051;
    // let grpc_client =  Arc::new(
        // grpc::ClientBuilder::new("::1", port) .build()
        // .unwrap(),
    // );
    // let client = AccountShareClient::with_client(grpc_client);

    let mut client = share_account_client::ShareAccountClient::connect("http://[::1]:50051").await.unwrap();
    let req = tonic::Request::new(
        Empty {}
    );

    // send the request
    let resp = client.web_request(req).await;
    // let a = resp.unwrap().into_inner();

    // while let Some(note) = a.message().await.unwrap() {
        // println!("NOTE = {:?}", note);
    // }
    let res = match resp {
        Ok(r) => r.into_inner().urlscheme,
        Err(_) => "error".to_owned(),
    };
    return res.to_string()
}


// async fn sleepy(Seconds(seconds): Seconds) -> Result<impl warp::Reply, Infallible> {

    // let url = get_grpc_result().await;
    // println!("11111");

    // tokio::time::sleep(Duration::from_secs(seconds)).await;
    // Ok(format!("I waited {} seconds! {}", seconds, url))
// }

// /// A newtype to enforce our maximum allowed seconds.
// struct Seconds(u64);

// impl FromStr for Seconds {
    // type Err = ();
    // fn from_str(src: &str) -> Result<Self, Self::Err> {
        // src.parse::<u64>().map_err(|_| ()).and_then(|num| {
            // if num <= 5 {
                // Ok(Seconds(num))
            // } else {
                // Err(())
            // }
        // })
    // }
// }


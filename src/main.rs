#![deny(warnings)]

use std::sync::Arc;
use std::convert::Infallible;
use std::str::FromStr;
use std::time::Duration;
use warp::Filter;

use share_account::service::*;
use share_account::service_grpc::*;

use grpc::ClientStub;
// use grpc::ClientStubExt;
// use futures::executor;


#[tokio::main]
async fn main() {
    // Match `/:Seconds`...
    let routes = warp::path::param()
        // and_then create a `Future` that will simply wait N seconds...
        .and_then(sleepy);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

async fn sleepy(Seconds(seconds): Seconds) -> Result<impl warp::Reply, Infallible> {

    let port = 50051;
    // let client_conf = Default::default();

    // create a client
    // let grpc_client = AccountShareClient::new_plain("::1", port, client_conf).unwrap();
    let grpc_client =  Arc::new(
        grpc::ClientBuilder::new("::1", port)
        .build()
        .unwrap(),
    );
    let client = AccountShareClient::with_client(grpc_client);


    // let mut client = AccountShareClient::connect("http://[::1]:50051").await?;
    let req = Empty::new();

    // send the request
    let resp = client
        .web_request(grpc::RequestOptions::new(), req)
        .completed();

    println!("{:#?}", resp);
    // let client = AccountShareClient::new_plain("::1", port, client_conf).unwrap();

    println!("11111");

    tokio::time::sleep(Duration::from_secs(seconds)).await;
    Ok(format!("I waited {} seconds!", seconds))
}

/// A newtype to enforce our maximum allowed seconds.
struct Seconds(u64);

impl FromStr for Seconds {
    type Err = ();
    fn from_str(src: &str) -> Result<Self, Self::Err> {
        src.parse::<u64>().map_err(|_| ()).and_then(|num| {
            if num <= 5 {
                Ok(Seconds(num))
            } else {
                Err(())
            }
        })
    }
}


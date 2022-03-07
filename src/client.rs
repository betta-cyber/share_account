use std::env;
use std::sync::Arc;

// importing generated gRPC code
use share_account::service_grpc::*;
// importing types for messages
use share_account::service::*;

use grpc::ClientStub;
// use grpc::ClientStubExt;
use futures::executor;

fn main() {

    let name = "anshul";
    let port = 50051;
    let client_conf = Default::default();

    // create a client
    let grpc_client =  Arc::new(
        grpc::ClientBuilder::new("::1", port)
        .build()
        .unwrap(),
    );
    let client = AccountShareClient::with_client(grpc_client);

    // create request
    let mut req = Empty::new();

    // send the request
    let resp = client
        .web_request(grpc::RequestOptions::new(), req)
        .join_metadata_result();

    // let mut stream = resp.drop_metedata();
    while let Some(f) = stream.next().await {
        let f = f.expect("iii");
        println!("ffff");
    }


    // wait for response
    println!("{:?}", executor::block_on(resp));
}

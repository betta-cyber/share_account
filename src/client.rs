use std::env;
use std::sync::Arc;

// importing generated gRPC code
use share_account::share_account_grpc::*;
// importing types for messages
use share_account::share_account::*;

use grpc::ClientStub;
use grpc::ClientStubExt;
use futures::executor;

fn main() {

    let name = "anshul";
    let port = 50051;
    let client_conf = Default::default();

    // create a client
    let client=GreeterClient::new_plain("::1", port, client_conf).unwrap();

    // create request
    let mut req = HelloRequest::new();
    req.set_name(name.to_string());

    // send the request
    let resp = client
        .say_hello(grpc::RequestOptions::new(), req)
        .join_metadata_result();

    // wait for response
    println!("{:?}", executor::block_on(resp));
}

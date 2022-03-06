use std::env;
use std::thread;

use share_account::share_account::*;
use share_account::share_account_grpc::*;

use grpc::ServerRequestSingle;
use grpc::ServerResponseUnarySink;
use grpc::ServerHandlerContext;

struct GreeterImpl;

impl Greeter for GreeterImpl {
    // rpc for service
    fn say_hello(
        &self,
        _: ServerHandlerContext,
        req: ServerRequestSingle<HelloRequest>,
        resp: ServerResponseUnarySink<HelloReply>,
    ) -> grpc::Result<()> {
        // create Response
        let mut r = HelloReply::new();
        let name = if req.message.get_name().is_empty() {
            "world"
        } else {
            req.message.get_name()
        };
        // sent the response
        println!("greeting request from {}", name);
        r.set_message(format!("Hello {}", name));
        resp.finish(r)
    }
}

fn main() {
    let port =50051;
    // creating server
    let mut server = grpc::ServerBuilder::new_plain();
    // adding port to server for http
    server.http.set_port(port);
    // adding say service to server
    server.add_service(GreeterServer::new_service_def(GreeterImpl));
    // running the server
    let _server = server.build().expect("server");
    println!(
        "greeter server started on port {}",
        port,
    );
    // stopping the program from finishing
    loop {
        std::thread::park();
    }
}

use std::env;
use std::thread;
use std::time;

use share_account::service::*;
use share_account::service_grpc::*;

use grpc::ServerRequestSingle;
use grpc::ServerResponseSink;
use grpc::ServerHandlerContext;

struct AccountShareImpl;

impl AccountShare for AccountShareImpl {
    // rpc for service
    fn web_request(
        &self,
        _: ServerHandlerContext,
        _req: ServerRequestSingle<Empty>,
        mut resp: ServerResponseSink<Data>,
    ) -> grpc::Result<()> {
        // create Response
        let mut r = Data::new();
        let name = "world";
        // sent the response
        println!("greeting request from {}", name);
        r.set_urlscheme(format!("Hello {}", name));

        thread::sleep(time::Duration::from_millis(4000));

        resp.send_data(r.clone())
    }

    fn phone_request(
        &self,
        _: ServerHandlerContext,
        _req: ServerRequestSingle<Empty>,
        mut resp: ServerResponseSink<Data>,
    ) -> grpc::Result<()> {
        // create Response
        let mut r = Data::new();
        let name = "world";
        // sent the response
        println!("greeting request from {}", name);
        r.set_urlscheme(format!("Hello {}", name));
        resp.send_data(r.clone())
    }

}

fn main() {
    let port =50051;
    // creating server
    let mut server = grpc::ServerBuilder::new_plain();
    // adding port to server for http
    server.http.set_port(port);
    // adding say service to server
    server.add_service(AccountShareServer::new_service_def(AccountShareImpl));
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


extern crate auth;
extern crate protobuf;
extern crate grpc;

//extern crate futures_cpupool;

use auth::interface::auth::*;
use grpc::Server;
use auth::interface::auth_grpc::authority;
use auth::interface::auth_grpc::*;

use std::thread;

struct Auth;

impl authority for Auth {
    fn auth (&self, o : ::grpc::RequestOptions, p : auth::Transation) -> ::grpc::SingleResponse<auth::State> {
        let mut resp = auth::State::new();
        let req = p.get_tx();
        let res = format!("auth tx : {}", req);

        resp.set_state(res);
        grpc::SingleResponse::completed(resp)
    }
}

fn main() {
    println!("Running Auth Service...");

    let mut server = grpc::ServerBuilder::new_plain();

    server.http.set_addr("0.0.0.0:30303");
    server.http.set_cpu_pool_threads(4);
    server.add_service(authorityServer::new_service_def(Auth));
    let _server : Server = server.build().expect("server");

    loop {
        thread::park();
    }
}

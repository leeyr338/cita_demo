
extern crate auth;
extern crate protobuf;
extern crate grpc;
extern crate rand;

use grpc::Server;
use auth::interface::auth_grpc::authority;
use auth::interface::auth_grpc::*;

use std::thread;
use rand::Rng;

struct Auth {
    inner_id : u32,
}

impl authority for Auth {
    fn auth (&self, _o : ::grpc::RequestOptions, p : auth::Transation) -> ::grpc::SingleResponse<auth::State> {
        let mut resp = auth::State::new();
        let req = p.get_tx();
        let res = format!("{} --> Auth ID_{} respone ok", req, self.inner_id);

        resp.set_state(res);
        grpc::SingleResponse::completed(resp)
    }
}

fn main() {
    println!("Running Auth Service...");

    let mut server = grpc::ServerBuilder::new_plain();

    server.http.set_addr("0.0.0.0:30303").expect("cannot setup auth service");
    server.http.set_cpu_pool_threads(4);
    server.add_service(authorityServer::new_service_def(Auth{inner_id : rand::thread_rng().gen::<u32>()}));
    let _server : Server = server.build().expect("server");

    loop {
        thread::park();
    }
}

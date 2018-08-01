
extern crate auth_interface;
extern crate protobuf;
extern crate grpc;
extern crate rand;

use grpc::Server;
use auth_interface::auth_grpc::Auth;
use auth_interface::auth_grpc::*;
use auth_interface::auth;

use std::thread;
use rand::Rng;

struct my_auth {
    inner_id : u32,
}

impl Auth for my_auth {
    fn send_transaction (&self, _o : ::grpc::RequestOptions, p : auth::SendTransactionReq) -> ::grpc::SingleResponse<auth::SendTransactionRes> {
        let mut resp = auth::SendTransactionRes::new();
        let crypto_type = p.get_untx().get_crypto();
        let res = format!("{} --> Auth ID_{} respone ok", crypto_type, self.inner_id);

        resp.set_tx_hash(res);
        grpc::SingleResponse::completed(resp)
    }

    fn pack_transactions (&self, _o : ::grpc::RequestOptions, p : auth::PackTransactionsReq) -> ::grpc::SingleResponse<auth::PackTransactionsRes> {

    }
}

fn main() {
    println!("Running Auth Service...");

    let mut server = grpc::ServerBuilder::new_plain();

    server.http.set_addr("0.0.0.0:30303").expect("cannot setup auth service");
    server.http.set_cpu_pool_threads(4);
    server.add_service(authorityServer::new_service_def(my_auth{inner_id : rand::thread_rng().gen::<u32>()}));
    let _server : Server = server.build().expect("server");

    loop {
        thread::park();
    }
}


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


// ==== sevice framework ====
pub trait AuthServiceIntf {
    fn init (&self);
}

pub struct AuthService {
    inner_id : u32,
    //service_hdl : AuthServiceIntf,
}

impl AuthService {

    fn new_service<T: AuthServiceIntf>(svc: T) -> AuthService {
        let auth_svc = AuthService {inner_id : rand::thread_rng().gen::<u32>()};

        let mut server = grpc::ServerBuilder::new_plain();

        server.http.set_addr("0.0.0.0:30303").expect("cannot setup auth service");
        server.http.set_cpu_pool_threads(4);
        server.add_service(AuthServer::new_service_def(AuthService{inner_id : rand::thread_rng().gen::<u32>()}));
        let _server : Server = server.build().expect("server");

        svc.init();
        thread::park();
        auth_svc
    }

    fn run (&self) {
        loop {
            thread::park();
        }
    }
}

impl Auth for AuthService {

    fn add_unverify_tx(&self, o: ::grpc::RequestOptions, p: auth::AddUnverifyTxReq) -> ::grpc::SingleResponse<auth::AddUnverifyTxRes> {
        println!("Enter add_unverify_tx!");
        let mut resp = auth::AddUnverifyTxRes::new();

        let res = format!("Auth ID_{} respone ok", self.inner_id);

        resp.set_tx_res(res);
        grpc::SingleResponse::completed(resp)
    }

    fn verify_batch_txs(&self, o: ::grpc::RequestOptions, p: auth::VerifyBatchTxsReq) -> ::grpc::SingleResponse<auth::RpcStatus> {
        let mut resp = auth::RpcStatus::new();
        let res = format!("Auth ID_{} respone ok", self.inner_id);

        resp.set_tx_res(res);
        grpc::SingleResponse::completed(resp)
    }

    fn get_txs_hashes(&self, o: ::grpc::RequestOptions, p: auth::GetTxsHashesReq) -> ::grpc::SingleResponse<auth::GetTxsHashesRes>{
        let mut resp = auth::GetTxsHashesRes::new();
        let res = format!("Auth ID_{} respone ok", self.inner_id);

        resp.set_tx_res(res);
        grpc::SingleResponse::completed(resp)
    }

    fn store_txs(&self, o: ::grpc::RequestOptions, p: auth::StoreTxsReq) -> ::grpc::SingleResponse<auth::StoreTxsRes>{
        let mut resp = auth::StoreTxsRes::new();
        let res = format!("Auth ID_{} respone ok", self.inner_id);

        resp.set_tx_res(res);
        grpc::SingleResponse::completed(resp)
    }

    fn clean_txs_pool(&self, o: ::grpc::RequestOptions, p: auth::CleanTxsPoolReq) -> ::grpc::SingleResponse<auth::RpcStatus>{
        let mut resp = auth::RpcStatus::new();
        let res = format!("Auth ID_{} respone ok", self.inner_id);

        resp.set_tx_res(res);
        grpc::SingleResponse::completed(resp)
    }

}
// === end service framework ===

// === implement service ===

struct AuthImpl {

}

impl AuthServiceIntf for AuthImpl {
    fn init (&self) {
        println!("Init Auth Service...");

        println!("Init Auth Service end!");
    }
}

fn main() {
    println!("Running Auth Service...");

    // let mut server = grpc::ServerBuilder::new_plain();
    //
    // server.http.set_addr("0.0.0.0:30303").expect("cannot setup auth service");
    // server.http.set_cpu_pool_threads(4);
    // server.add_service(AuthServer::new_service_def(AuthService{inner_id : rand::thread_rng().gen::<u32>()}));
    // let _server : Server = server.build().expect("server");

    let auth = AuthImpl {};
    let auth_service = AuthService::new_service(auth);

    auth_service.run();

}

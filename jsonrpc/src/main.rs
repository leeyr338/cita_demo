
#![deny(warnings)]
extern crate hyper;
extern crate pretty_env_logger;
extern crate auth_interface;
extern crate protobuf;
extern crate grpc;
// extern crate futures;

#[macro_use]
extern crate jsonrpc_derive;

use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};

use hyper::{Body, Response, Server, Method, StatusCode};
use hyper::service::service_fn_ok;
use hyper::rt::{self, Future};

use auth_interface::auth;
use auth_interface::auth_grpc::*;

#[derive(GrpcClient)]
#[grpc(tgt_svc_name = "Auth")]
#[grpc(ip = "0.0.0.0")]
#[grpc(port = "30303")]
pub struct AuthGrpcClient;

fn main() {
    println!("###Running jsonrpc service...", );
    pretty_env_logger::init();

    let addr = ([127,0,0,1], 3000).into();

    let counter = Arc::new(AtomicUsize::new(0));

    let new_service = move || {
        println!("conect client start...");
        //let client = AuthClient::new_plain("auth", 6666, Default::default()).unwrap();

        let client = AuthGrpcClient::new();;

        let counter = counter.clone();
        println!("conect client end");

        service_fn_ok(move |req| {
            let count = counter.fetch_add(1, Ordering::AcqRel);
            let mut response = Response::new(Body::empty());

            match (req.method(), req.uri().path()) {
                (&Method::GET, "/") => {
                    *response.body_mut() = Body::from("...welcome to cita demo...");
                }
                (&Method::POST, "/tx") => {
                    let mut tx = auth::AddUnverifyTxReq::new();

                    tx.set_untx("new transation".to_string());
                    let resp = client.add_unverify_tx(grpc::RequestOptions::new(), tx);
                    match resp.wait() {
                        Ok(rsp) => *response.body_mut() = Body::from(format!("Request #{} : {:?}", count, rsp.1.get_tx_res())),
                        Err(e) => *response.body_mut() = Body::from(format!("ERROR : Request #{} : {:?}", count, e)),
                    }

                }
                _ => {
                    *response.status_mut() = StatusCode::NOT_FOUND;
                }
            }
            response
        })
    };

    let server = Server::bind(&addr)
        .serve(new_service)
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Listening on http://{}", addr);

    rt::run(server);
}

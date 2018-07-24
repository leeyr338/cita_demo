#![deny(warnings)]
extern crate hyper;
extern crate pretty_env_logger;

use hyper::{Body, Response, Server};
use hyper::service::service_fn_ok;
use hyper::rt::{self, Future};

static PHRASE : &'static [u8] = b"Hello world, leeyr";

fn main() {
    pretty_env_logger::init();

    let addr = ([0,0,0,0], 3000).into();

    let new_service = || {
        service_fn_ok(|_| {
            Response::new(Body::from(PHRASE))
        })
    };

    let server = Server::bind(&addr)
                 .serve(new_service)
                 .map_err(|e| eprintln!("server error {}", e));



    println!("lisenting on http://{}", addr);
    rt::run(server);
}

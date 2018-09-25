
extern crate serde;
extern crate httparse;
extern crate toml;
extern crate tokio_core;
extern crate net2;

#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

mod config;
mod http_server;

use tokio_core::reactor::Core;

fn main() {
    println!("###Running jsonrpc service...", );
    let config = config::Config::new("./jsonrpc.toml");

    if config.http_config.enable {
        let http_config = config.http_config.clone();

        let addr = http_config.listen_ip.clone() + ":" + &http_config.listen_port.clone().to_string();

        let addr = addr.clone().parse().unwrap();
        let core = Core::new().unwrap();
        let handle = core.handle();

        let listener = http_server::listener(&addr, &handle).unwrap();

        println!("Listening on {}", addr);
    }

}

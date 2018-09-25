
use net2;
use std::net::SocketAddr;
use tokio_core::net::TcpListener;
use tokio_core::reactor::{Core, Handle, Timeout};

use std::io;

const TCP_BACKLOG: i32 = 1024;

pub fn listener (addr: &SocketAddr, handle: &Handle) -> io::Result<TcpListener> {
    let listener = match *addr {
        SocketAddr::V4(_) => net2::TcpBuilder::new_v4()?,
        SocketAddr::V6(_) => net2::TcpBuilder::new_v6()?,
    };
    configure_tcp(&listener)?;
    listener
        .listen(TCP_BACKLOG)
        .and_then(|l| TcpListener::from_listener(l, addr, handle))
}

fn configure_tcp(tcp: &net2::TcpBuilder) -> io::Result<()> {
    use net2::unix::*;
    tcp.reuse_port(true)?;
    Ok(())
}

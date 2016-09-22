extern crate bound_tcp_stream;

use bound_tcp_stream::BoundTcpStream;
use std::io::prelude::*;
use std::net::{Ipv4Addr, SocketAddrV4, SocketAddr};

fn main() {
    let source_addr = SocketAddrV4::new(Ipv4Addr::new(10, 240, 34, 166), 0);
    let dest_addr = SocketAddrV4::new(Ipv4Addr::new(10, 240, 32, 1), 22);

    let mut socket: BoundTcpStream = BoundTcpStream::new(SocketAddr::V4(source_addr)).unwrap();
    let mut stream = socket.connect(SocketAddr::V4(dest_addr)).unwrap();

    let mut buffer = [0; 10];
    println!("{}", stream.read(&mut buffer).unwrap());
    println!("{:?}", String::from_utf8_lossy(&buffer));
}

extern crate bound_tcp_stream;

use bound_tcp_stream::BoundTcpStream;
use std::io::prelude::*;

fn main() {
    let source_addr = "10.240.34.166:0";
    let dest_addr = "10.240.32.1:22";

    let mut stream = BoundTcpStream::new(source_addr, dest_addr).unwrap();

    let mut buffer = [0; 10];
    println!("{}", stream.read(&mut buffer).unwrap());
    println!("{:?}", String::from_utf8_lossy(&buffer));
}

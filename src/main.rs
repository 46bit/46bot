extern crate irc;
extern crate libc;

use std::default::Default;
use irc::client::prelude::*;
use std::thread;
use std::time;

use std::mem;
use std::os::unix::prelude::*;
use std::net;

extern crate bound_tcp_stream;

use bound_tcp_stream::BoundTcpStream;
use std::io::prelude::*;
use std::net::{Ipv4Addr, SocketAddrV4, SocketAddr};

fn main() {
    let source_addr = SocketAddrV4::new(Ipv4Addr::new(10, 240, 34, 166), 0);
    let dest_addr = SocketAddrV4::new(Ipv4Addr::new(185, 30, 166, 38), 6667);

    //run(0, SocketAddr::V4(source_addr), SocketAddr::V4(dest_addr));

    let nums = vec![1, 2, 3];

for i in 0..nums.len() {
    println!("{}", nums[i]);
}

    let

    for
    for i in 0..29 {
        thread::spawn(move || {
            run(i, SocketAddr::V4(source_addr), SocketAddr::V4(dest_addr));
        });
        thread::sleep(time::Duration::from_millis(2000));
    }

    loop {
        thread::sleep(time::Duration::from_millis(500));
    }
}

fn run(i: i32, source_addr: SocketAddr, dest_addr: SocketAddr) {
    let config = Config {
        nickname: Some(format!("\\46bot{}", i)),
        channels: Some(vec![format!("#cs-york-dev")]),
        source_addr: Some(source_addr),
        dest_addr: Some(dest_addr),
        .. Default::default()
    };
    let server = IrcServer::from_config(config).unwrap();
    server.identify().unwrap();
    for message in server.iter() {
        let message = message.unwrap(); // We'll just panic if there's an error.
        print!("{}", message);
        match message.command {
            Command::PRIVMSG(ref target, ref msg) => if msg.contains("pickles") {
                server.send_privmsg(target, "Hi!").unwrap();
            },
            _ => (),
        }
    }
}

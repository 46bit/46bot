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
use std::net::{IpAddr, Ipv4Addr, SocketAddrV4, SocketAddr};

fn main() {
    let dest_addr = SocketAddrV4::new(Ipv4Addr::new(195, 154, 200, 232), 6667);

    let ips = vec![
        Ipv4Addr::new(192, 168, 1, 162),
    ];
    let mut source_addrs: Vec<SocketAddrV4> = vec![];
    for i in 0..ips.len() {
        source_addrs.push(SocketAddrV4::new(ips[i], 0));
    }

    for i in 0..29 {
        for j in 0..source_addrs.len() {
            let source_addr = source_addrs[j];
            let bot_id = j * 100 + i;
            thread::spawn(move || {
                run(bot_id as i32, SocketAddr::V4(source_addr), SocketAddr::V4(dest_addr));
            });
            thread::sleep(time::Duration::from_millis(10000));
        }
    }

    loop {
        thread::sleep(time::Duration::from_millis(500));
    }
}

fn run(i: i32, source_addr: SocketAddr, dest_addr: SocketAddr) {
    let config = Config {
        nickname: Some(format!("zzbot{}", i)),
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

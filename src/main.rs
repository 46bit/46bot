extern crate irc;
extern crate libc;

use std::default::Default;
use irc::client::prelude::*;
use std::thread;
use std::time;
use std::str::FromStr;
use std::mem;
use std::os::unix::prelude::*;
use std::net;

extern crate bound_tcp_stream;

use bound_tcp_stream::BoundTcpStream;
use std::io::prelude::*;
use std::net::{IpAddr, Ipv4Addr, SocketAddrV4, SocketAddr, Ipv6Addr, SocketAddrV6};

fn main() {
    let dest_addr = SocketAddrV6::new(Ipv6Addr::from_str("2a00:1a28:1100:11::42").unwrap(), 6667, 0, 0);

    let ips = vec![
        Ipv6Addr::from_str("2604:a880:800:10::19e0:7000").unwrap(),
        Ipv6Addr::from_str("2604:a880:800:10::19e0:7002").unwrap(),
        Ipv6Addr::from_str("2604:a880:800:10::19e0:7003").unwrap(),
        Ipv6Addr::from_str("2604:a880:800:10::19e0:7004").unwrap(),
        Ipv6Addr::from_str("2604:a880:800:10::19e0:7005").unwrap(),
        Ipv6Addr::from_str("2604:a880:800:10::19e0:7006").unwrap(),
        Ipv6Addr::from_str("2604:a880:800:10::19e0:7007").unwrap(),
        Ipv6Addr::from_str("2604:a880:800:10::19e0:7008").unwrap(),
        Ipv6Addr::from_str("2604:a880:800:10::19e0:7009").unwrap(),
        Ipv6Addr::from_str("2604:a880:800:10::19e0:700a").unwrap(),
        Ipv6Addr::from_str("2604:a880:800:10::19e0:700b").unwrap(),
        Ipv6Addr::from_str("2604:a880:800:10::19e0:700c").unwrap(),
        Ipv6Addr::from_str("2604:a880:800:10::19e0:700d").unwrap(),
        Ipv6Addr::from_str("2604:a880:800:10::19e0:700e").unwrap(),
        Ipv6Addr::from_str("2604:a880:800:10::19e0:700f").unwrap(),
    ];
    let mut source_addrs: Vec<SocketAddrV6> = vec![];
    for i in 0..ips.len() {
        source_addrs.push(SocketAddrV6::new(ips[i], 0, 0, 0));
    }

    let mut bot_count = 0;
    for i in 0..29 {
        for j in 0..source_addrs.len() {
            let source_addr = source_addrs[j];
            thread::spawn(move || {
                let bot_id = bot_count;
                run(bot_id as u64, SocketAddr::V6(source_addr), SocketAddr::V6(dest_addr));
            });
            thread::sleep(time::Duration::from_millis(1000));
            bot_count += 1;
        }
        thread::sleep(time::Duration::from_millis(3000));
    }

    loop {
        thread::sleep(time::Duration::from_millis(500));
    }
}

fn run(bot_id: u64, source_addr: SocketAddr, dest_addr: SocketAddr) {
    let config = Config {
        nickname: Some(format!("\\46boit{}", bot_id)),
        channels: Some(vec![format!("#46bots")]),
        source_addr: Some(source_addr),
        dest_addr: Some(dest_addr),
        .. Default::default()
    };
    let server = IrcServer::from_config(config).unwrap();
    server.identify().unwrap();

    for message in server.iter() {
        let message = message.unwrap(); // We'll just panic if there's an error.
        print!("{:?}", message);

        match message.command {
            Command::PRIVMSG(ref target, ref msg) => {
                if msg.contains("\\46bots: leave") {
                    thread::sleep(time::Duration::from_millis(100 * bot_id));
                    server.send_part(target).unwrap();
                }
            },
            _ => (),
        }

        if message.prefix.is_some() && message.prefix.unwrap() == "_46bit!~fortysix@pdpc/supporter/student/mmokrysz" {
            match message.command {
                Command::PRIVMSG(ref target, ref msg) => {
                    if msg.contains("\\46bots: quit") {
                        thread::sleep(time::Duration::from_millis(100 * bot_id));
                        server.send_quit("_46bites the dust.").unwrap();
                    }
                },
                _ => (),
            }
        }
    }
}

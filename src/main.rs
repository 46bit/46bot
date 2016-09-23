extern crate irc;
extern crate libc;
extern crate bound_tcp_stream;

use std::default::Default;
use irc::client::prelude::*;
use std::thread;
use std::time;
use std::str::FromStr;
use std::net::{IpAddr, SocketAddr};

fn main() {
    let dest_addr = SocketAddr::new(IpAddr::from_str("2a00:1a28:1100:11::42").unwrap(), 6667);

    let ips = vec![
        IpAddr::from_str("2604:a880:800:10::19e0:7000").unwrap(),
        IpAddr::from_str("2604:a880:800:10::19e0:7002").unwrap(),
        IpAddr::from_str("2604:a880:800:10::19e0:7003").unwrap(),
        IpAddr::from_str("2604:a880:800:10::19e0:7004").unwrap(),
        IpAddr::from_str("2604:a880:800:10::19e0:7005").unwrap(),
        IpAddr::from_str("2604:a880:800:10::19e0:7006").unwrap(),
        IpAddr::from_str("2604:a880:800:10::19e0:7007").unwrap(),
        IpAddr::from_str("2604:a880:800:10::19e0:7008").unwrap(),
        IpAddr::from_str("2604:a880:800:10::19e0:7009").unwrap(),
        IpAddr::from_str("2604:a880:800:10::19e0:700a").unwrap(),
        IpAddr::from_str("2604:a880:800:10::19e0:700b").unwrap(),
        IpAddr::from_str("2604:a880:800:10::19e0:700c").unwrap(),
        IpAddr::from_str("2604:a880:800:10::19e0:700d").unwrap(),
        IpAddr::from_str("2604:a880:800:10::19e0:700e").unwrap(),
        IpAddr::from_str("2604:a880:800:10::19e0:700f").unwrap(),
    ];
    let mut source_addrs: Vec<SocketAddr> = vec![];
    for i in 0..ips.len() {
        source_addrs.push(SocketAddr::new(ips[i], 0));
    }

    let mut bot_count = 0;
    for _ in 0..29 {
        for j in 0..source_addrs.len() {
            let source_addr = source_addrs[j];
            thread::spawn(move || {
                let bot_id = bot_count;
                run(bot_id as u64, source_addr, dest_addr);
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
        nickname: Some(format!("\\46bot{}", bot_id)),
        channels: Some(vec![format!("#46bots")]),
        source_addr: Some(source_addr),
        dest_addr: Some(dest_addr),
        .. Default::default()
    };
    let server = IrcServer::from_config(config).unwrap();
    server.identify().unwrap();

    for message in server.iter() {
        let message = message.unwrap(); // We'll just panic if there's an error.
        //print!("{:?}", message);

        match message.command {
            Command::PRIVMSG(ref target, ref msg) => {
                if msg == "\\46bots: leave" {
                    thread::sleep(time::Duration::from_millis(50 * bot_id));
                    server.send_part(target).unwrap();
                }
            },
            _ => (),
        }

        if message.prefix.is_some() {
            let prefix = message.prefix.unwrap();
            if prefix == "_46bit!~fortysix@pdpc/supporter/student/mmokrysz" {
                match message.command {
                    Command::PRIVMSG(ref target, ref msg) => {
                        print!("prefix={:?} command={:?}", prefix, message.command);
                        if msg == "\\46bots: quit" {
                            thread::sleep(time::Duration::from_millis(50 * bot_id));
                            server.send_quit("_46bites the dust.").unwrap();
                            return;
                        } else if msg.starts_with("\\46bots: join ") {
                            thread::sleep(time::Duration::from_millis(50 * bot_id));
                            let (_, channel) = msg.split_at("\\46bots: join ".len());
                            server.send_join(channel).unwrap();
                        } else if msg.starts_with("\\46bots: say ") {
                            thread::sleep(time::Duration::from_millis(50 * bot_id));
                            let (_, words) = msg.split_at("\\46bots: say ".len());
                            server.send_notice(target, words).unwrap();
                        }
                    },
                    _ => (),
                }
            }
        }
    }
}

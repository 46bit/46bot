use utils::*;
use tcp_socket::*;

use std::io::{Result, Error, ErrorKind};
use std::net::{TcpStream, SocketAddr, ToSocketAddrs};
use libc;

pub struct BoundTcpStream;

impl BoundTcpStream {
    pub fn new<A: ToSocketAddrs>(source_addr: A, dest_addr: A) -> Result<TcpStream> {
        // 1. Find a binding source socket.
        // 2. Find a connecting destination socket.
        // @TODO @DEBUG: This assumes a socket which fails to connect() can *always* be used to
        //               try another connect()ion.

        let socket = Self::each_addr(source_addr, Self::try_socket_and_bind)?;
        match Self::each_addr_with_param(dest_addr, &socket, Self::try_connect) {
            Ok(_) => Ok(socket.as_tcp_stream()),
            Err(e) => Err(e),
        }
    }

    fn try_socket_and_bind(source_addr: &SocketAddr) -> Result<TcpSocket> {
        let socket = TcpSocket::new(IP::from_socket_addr(*source_addr))?;
        socket.bind(*source_addr)?;
        Ok(socket)
    }

    fn try_connect(dest_addr: &SocketAddr, socket: &TcpSocket) -> Result<libc::c_int> {
        socket.connect(*dest_addr)
    }

    // From std::net::mod each_addr
    fn each_addr<A: ToSocketAddrs, F, T>(addr: A, mut f: F) -> Result<T>
        where F: FnMut(&SocketAddr) -> Result<T>
    {
        let mut last_err = None;
        for addr in addr.to_socket_addrs()? {
            match f(&addr) {
                Ok(l) => return Ok(l),
                Err(e) => last_err = Some(e),
            }
        }
        Err(last_err.unwrap_or_else(|| {
            Error::new(ErrorKind::InvalidInput,
                       "could not resolve to any addresses")
        }))
    }

    // From std::net::mod each_addr
    fn each_addr_with_param<A: ToSocketAddrs, B, F, T>(addr: A, param: &B, mut f: F) -> Result<T>
        where F: FnMut(&SocketAddr, &B) -> Result<T>
    {
        let mut last_err = None;
        for addr in addr.to_socket_addrs()? {
            match f(&addr, param) {
                Ok(l) => return Ok(l),
                Err(e) => last_err = Some(e),
            }
        }
        Err(last_err.unwrap_or_else(|| {
            Error::new(ErrorKind::InvalidInput,
                       "could not resolve to any addresses")
        }))
    }
}

// Heavily based upon
//   https://github.com/rust-lang/rust/blob/master/src/libstd/sys/unix/net.rs

#![feature(question_mark)]
#![feature(ipv6_to_octets)]

extern crate libc;

use std::mem;
use std::io::{self, Error, ErrorKind};
use std::net::{TcpStream,SocketAddr,ToSocketAddrs,SocketAddrV4,SocketAddrV6};
use std::os::unix::prelude::*;
use libc::{c_int, socklen_t, sockaddr_storage, sockaddr_in, sockaddr_in6, listen};

// From std::sys::unix::mod IsMinusOne, cvt
pub fn cvt(t: c_int) -> io::Result<c_int> {
    if t == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(t)
    }
}

// From std::net::mod each_addr
fn each_addr<A: ToSocketAddrs, F, T>(addr: A, mut f: F) -> io::Result<T>
    where F: FnMut(&SocketAddr) -> io::Result<T>
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

// See below for the usage of SOCK_CLOEXEC, but this constant is only defined on
// Linux currently (e.g. support doesn't exist on other platforms). In order to
// get name resolution to work and things to compile we just define a dummy
// SOCK_CLOEXEC here for other platforms. Note that the dummy constant isn't
// actually ever used (the blocks below are wrapped in `if cfg!` as well.
#[cfg(target_os = "linux")]
use libc::SOCK_CLOEXEC;
#[cfg(not(target_os = "linux"))]
const SOCK_CLOEXEC: c_int = 0;

extern {
    fn bind(sockfd: c_int, sockaddr: *mut sockaddr_storage, addrlen: socklen_t) -> c_int;
    fn connect(sockfd: c_int, sockaddr: *mut sockaddr_storage, addrlen: socklen_t) -> c_int;
}

pub struct TCPSocket(RawFd);

impl TCPSocket {
    pub fn new() {

    }

    fn new_raw(fam: c_int, ty: c_int) -> io::Result<RawFd> {
        unsafe {
            // On linux we first attempt to pass the SOCK_CLOEXEC flag to
            // atomically create the socket and set it as CLOEXEC. Support for
            // this option, however, was added in 2.6.27, and we still support
            // 2.6.18 as a kernel, so if the returned error is EINVAL we
            // fallthrough to the fallback.
            if cfg!(target_os = "linux") {
                match cvt(libc::socket(fam, ty | SOCK_CLOEXEC, 0)) {
                    Ok(fd) => return Ok(fd),
                    Err(ref e) if e.raw_os_error() == Some(libc::EINVAL) => {}
                    Err(e) => return Err(e),
                }
            }

            let fd = cvt(libc::socket(fam, ty, 0))?;
            // @TODO: fd.set_cloexec()?;
            Ok(fd)
        }
    }

    pub fn bind(&self, addr: SocketAddr) -> io::Result<c_int> {
        let (mut sockaddr, sockaddr_size) = self.sockaddr(addr);

        unsafe {
          match cvt(bind(self.0, &mut sockaddr, sockaddr_size as u32)) {
            Ok(t) => return Ok(t),
            Err(e) => return Err(e),
          }
        }
    }

    pub fn listen(&self) -> io::Result<c_int> {
        cvt(listen(self.0, 128))
    }

    pub fn connect(&self, addr: SocketAddr) -> io::Result<TcpStream> {
        let (mut sockaddr, sockaddr_size) = self.sockaddr(addr);

        // @TODO: Check result of TcpStream::from_raw_fd.
        unsafe {
            match cvt(connect(self.0, &mut sockaddr, sockaddr_size as u32)) {
                Ok(_) => return Ok(TcpStream::from_raw_fd(self.0)),
                Err(e) => return Err(e),
            }
        }
    }

    fn sockaddr(&mut self, addr: SocketAddr) -> (sockaddr_storage, usize) {
        match addr {
            SocketAddr::V4(v4) => self.sockaddr_in4(v4),
            SocketAddr::V6(v6) => self.sockaddr_in6(v6),
        }
    }

    fn sockaddr_in4(&mut self, addr: SocketAddrV4) -> (sockaddr_storage, usize) {
        let mut sockaddr: sockaddr_in;
        unsafe { sockaddr = mem::zeroed(); }

        sockaddr.sin_family = libc::AF_INET as libc::sa_family_t;
        sockaddr.sin_port = addr.port().to_be();
        sockaddr.sin_addr.s_addr = u32::from(*addr.ip()).to_be();

        unsafe { (*(&mut sockaddr as *mut _ as *mut sockaddr_storage), mem::size_of::<sockaddr_in>()) }
    }

    fn sockaddr_in6(&mut self, addr: SocketAddrV6) -> (sockaddr_storage, usize) {
        let mut sockaddr: sockaddr_in6;
        unsafe { sockaddr = mem::zeroed(); }

        sockaddr.sin6_family = libc::AF_INET6 as libc::sa_family_t;
        sockaddr.sin6_port = addr.port().to_be();
        sockaddr.sin6_flowinfo = addr.flowinfo();
        sockaddr.sin6_addr.s6_addr = addr.ip().octets();
        sockaddr.sin6_scope_id = addr.scope_id();

        unsafe { (*(&mut sockaddr as *mut _ as *mut sockaddr_storage), mem::size_of::<sockaddr_in6>()) }
    }
}

pub struct BoundTcpStream(RawFd);

impl BoundTcpStream {
    /// Bind to a local IP address and port, ready to then connect.
    ///
    /// `addr` is an address of the local host. Anything which implements
    /// `ToSocketAddrs` trait can be supplied for the address; see this trait
    /// documentation for concrete examples.
    pub fn new<A: ToSocketAddrs>(source_addr: A) -> io::Result<BoundTcpStream> {
        let source_addr = try!(source_addr.to_socket_addrs()).collect();

        // @TODO: https://github.com/rust-lang/rust/blob/master/src/libstd/sys/unix/net.rs#L61
        // REIMPLEMENT THAT ALL HERE
        let fam = match source_addr {
            SocketAddr::V4(_) => libc::AF_INET,
            SocketAddr::V6(_) => libc::AF_INET6,
        };
        let ty = libc::SOCK_STREAM;

        let mut socket = match BoundTcpStream::new_raw(fam, ty) {
            Ok(fd) => BoundTcpStream(fd),
            Err(e) => return Err(e),
        };

        // @TODO: Does this want setting all the time? Probably not. Perhaps a config option?
        unsafe {
          // @TODO: Avoid use of mem::transmute.
          let one_pointer = mem::transmute(&(1 as c_int));
          let c_int_size: u32 = mem::size_of::<c_int>() as u32;
          match cvt(libc::setsockopt(socket.0, libc::SOL_SOCKET, libc::SO_REUSEADDR, one_pointer, c_int_size)) {
            Ok(_) => {},
            Err(e) => return Err(e),
          }
        }

        try!(socket.bind(source_addr));

        Ok(socket)
    }

    fn new_raw(fam: c_int, ty: c_int) -> io::Result<RawFd> {
        unsafe {
            // On linux we first attempt to pass the SOCK_CLOEXEC flag to
            // atomically create the socket and set it as CLOEXEC. Support for
            // this option, however, was added in 2.6.27, and we still support
            // 2.6.18 as a kernel, so if the returned error is EINVAL we
            // fallthrough to the fallback.
            if cfg!(target_os = "linux") {
                match cvt(libc::socket(fam, ty | SOCK_CLOEXEC, 0)) {
                    Ok(fd) => return Ok(fd),
                    Err(ref e) if e.raw_os_error() == Some(libc::EINVAL) => {}
                    Err(e) => return Err(e),
                }
            }

            let fd = cvt(libc::socket(fam, ty, 0))?;
            // @TODO: fd.set_cloexec()?;
            Ok(fd)
        }
    }

    pub fn bind<A: ToSocketAddrs>(&mut self, source_addr: A) -> io::Result<c_int> {
        let (mut sockaddr, sockaddr_size) = match source_addr {
            SocketAddr::V4(v4) => self.sockaddr_in4(v4),
            SocketAddr::V6(v6) => self.sockaddr_in6(v6),
        };

        unsafe {
          match cvt(bind(self.0, &mut sockaddr, sockaddr_size as u32)) {
            Ok(t) => return Ok(t),
            Err(e) => return Err(e),
          }
        }
    }

    /// Opens a TCP connection to a remote host.
    ///
    /// `addr` is an address of the remote host. Anything which implements
    /// `ToSocketAddrs` trait can be supplied for the address; see this trait
    /// documentation for concrete examples.
    pub fn connect<A: ToSocketAddrs>(&mut self, dest_addr: A) -> io::Result<TcpStream> {
        // https://github.com/rust-lang/rust/blob/master/src/libstd/sys/common/net.rs#L184

        let (mut sockaddr, sockaddr_size) = match dest_addr {
            SocketAddr::V4(v4) => self.sockaddr_in4(v4),
            SocketAddr::V6(v6) => self.sockaddr_in6(v6),
        };

        // @TODO: Check result of TcpStream::from_raw_fd.
        unsafe {
          match cvt(connect(self.0, &mut sockaddr, sockaddr_size as u32)) {
              Ok(_) => return Ok(TcpStream::from_raw_fd(self.0)),
              Err(e) => return Err(e),
          }
        }
    }
}

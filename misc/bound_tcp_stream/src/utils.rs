use std::io;
use std::net;
use libc;

// From std::sys::unix::mod IsMinusOne, cvt
pub fn cvt(t: libc::c_int) -> io::Result<libc::c_int> {
    if t == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(t)
    }
}

pub enum IP {
    V4,
    V6,
}

impl IP {
    pub fn to_af_inet(&self) -> i32 {
        match *self {
            IP::V4 => libc::AF_INET,
            IP::V6 => libc::AF_INET6,
        }
    }

    pub fn from_ip_addr(ip_addr: net::IpAddr) -> IP {
        match ip_addr {
            net::IpAddr::V4(_) => IP::V4,
            net::IpAddr::V6(_) => IP::V6,
        }
    }

    pub fn from_socket_addr(socket_addr: net::SocketAddr) -> IP {
        match socket_addr {
            net::SocketAddr::V4(_) => IP::V4,
            net::SocketAddr::V6(_) => IP::V6,
        }
    }
}

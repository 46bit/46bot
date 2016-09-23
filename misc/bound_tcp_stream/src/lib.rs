// Heavily based upon
//   https://github.com/rust-lang/rust/blob/master/src/libstd/sys/unix/net.rs

#![feature(question_mark)]
#![feature(ipv6_to_octets)]

extern crate libc;

mod utils;
mod tcp_socket;
mod bound_tcp_stream;
// These aren't really separate modules. The separate files just make it more organised.
pub use utils::*;
pub use tcp_socket::*;
pub use bound_tcp_stream::*;

use std::marker;
use serde::{Serialize, Deserialize};
use rand::Rng;

use hexagon_grid::*;
use square_grid::*;
use triangle_grid::*;

#[derive(Serialize, Deserialize)]
pub enum World {
    HexagonGrid(HexagonGrid),
    SquareGrid(SquareGrid),
    TriangleGrid(TriangleGrid),
}

pub enum IPVersion {
  V4,
  V6,
}

pub trait BoundTcpStream {
    type Address;
    pub fn new(&self) -> Self;
    pub fn bind(&mut self, source_addr: &Self::Address) -> Self;
    pub fn connect(&mut self, dest_addr: &Self::Address) -> Self;
    pub fn to_tcp_stream(&mut self) -> TcpStream;
}

pub struct BoundTcpStreamV4(RawFd);

impl BoundTcpStream for BoundTcpStreamV4 {
    type Address = SocketAddrV4;

    pub fn new(&self) -> BoundTcpStreamV4 {

    }

    pub fn bind(&mut self, source_addr: &Self::Address) -> Self {

    }

    pub fn connect(&mut self, dest_addr: &Self::Address) -> Self {

    }

    pub fn to_tcp_stream(&mut self) -> TcpStream {

    }
}

pub struct BoundTcpStreamV6(RawFd);





pub struct V4BoundTcpStream(RawFd);

impl HexagonGrid {
    pub fn new(radius : usize) -> HexagonGrid {
        HexagonGrid{radius : radius}
    }
}

impl Grid for HexagonGrid {




pub trait Vector : Eq + Copy + Serialize + Deserialize {
    type Direction;// : Direction;
    fn distance(&self, other : &Self) -> usize;
    fn neighbour(&self, direction : &Self::Direction) -> Self;
    fn neighbours(&self) -> Vec<Self>;
}

pub trait Grid : Serialize + Deserialize {
    type Vector;// : Vector;
    fn dimensions(&self) -> Vec<isize>;
    fn is_within_bounds(&self, v : Self::Vector) -> bool;
    fn cells(&self) -> Vec<Self::Vector>;
    fn random_cell<R : Rng>(&self) -> Self::Vector;
}

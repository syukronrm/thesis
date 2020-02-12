// #[macro_use]
// extern crate float_cmp;
use float_cmp;

#[derive(Debug)]
pub struct Node {
  pub id: i16,
  pub lng: f32,
  pub lat: f32
}

#[derive(Debug)]
pub struct Edge<'a> {
  pub id: i32,
  pub ni: &'a Node,
  pub nj: &'a Node,
  pub len: f32
}

impl<'a> Edge<'a> {
  fn new(id: i32, ni: &'a Node, nj: &'a Node) -> Edge<'a> {
    Edge {
      id: id,
      ni: ni,
      nj: nj,
      len: 1.00
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn create_node() {
    let n1 = Node {id: 1, lng: 1.0, lat: 2.0};
    assert_eq!(n1.id, 1);
  }

  #[test]
  fn create_edge() {
    let n1 = Node {id: 1, lng: 3.0, lat: 0.0};
    let n2 = Node {id: 2, lng: 0.0, lat: 4.0};

    let e1 = Edge::new(1, &n1, &n2);

    assert!( float_cmp::approx_eq!(f32, e1.len, 1.0, ulps = 2) );
  }
}
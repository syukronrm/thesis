use petgraph::{Graph as PetGraph, Undirected};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub type DimensionIndex = i8;
pub type ObjectId = i32;
pub type Scope = HashMap<Vec<DimensionIndex>, Pair>;
pub type Pair = HashMap<ObjectId, Vec<Range>>;

#[allow(dead_code)]
pub struct Structure {
    graph: RefCell<PetGraph<Node, Edge, Undirected>>,
}

#[allow(dead_code)]
pub struct Node {
    id: i32,
    lng: f32,
    lat: f32,
}

#[allow(dead_code)]
pub struct Edge {
    id: i32,
    len: f32,
    ni: Rc<Node>,
    nj: Rc<Node>,
    objects: RefCell<Vec<Object>>,
    scope: RefCell<Scope>,
    sky_scope: RefCell<Scope>,
    two_sky_scope: RefCell<Scope>,
    d_sky_scope: RefCell<Scope>,
    k_sky_scope: RefCell<Scope>,
}

#[allow(dead_code)]
pub struct Object {
    id: ObjectId,
    attr: Vec<f32>,
}

#[allow(dead_code)]
pub struct Range {
    start: f32,
    end: f32,
}

use std::collections::HashMap;
use std::rc::Rc;

#[allow(dead_code)]
pub struct Structure {

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
    scope: Scope,
    sky_scope: Scope,
    two_scope: Scope,
    d_scope: Scope,
    k_skyline_scope: Scope,
}

#[allow(dead_code)]
pub type Scope = HashMap<Vec<i8>, Pair>;

#[allow(dead_code)]
pub struct Object {
    id: i32,
    attr: Vec<f32>,
}

#[allow(dead_code)]
pub type Pair = HashMap<i32, Vec<Range>>;       // HashMap<ObjectId,....>

#[allow(dead_code)]
pub struct Range {
    start: f32,
    end: f32,
}

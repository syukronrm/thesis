use std::cell::RefCell;
use std::collections::HashMap;

type DimensionIndex = i8;
type ObjectId = i32;
type Scope = HashMap<Vec<DimensionIndex>, Pair>;
type Pair = HashMap<ObjectId, Vec<Range>>;

#[derive(Debug)]
pub struct Edge {
    id: i32,
    len: f32,
    objects: RefCell<Vec<Object>>,
    scope: RefCell<Scope>,
    sky_scope: RefCell<Scope>,
    two_sky_scope: RefCell<Scope>,
    d_sky_scope: RefCell<Scope>,
    k_sky_scope: RefCell<Scope>,
}

impl Edge {
    pub fn new(id: i32, len: f32) -> Edge {
        Edge {
            id,
            len,
            objects: RefCell::new(Vec::new()),
            scope: RefCell::new(HashMap::new()),
            sky_scope: RefCell::new(HashMap::new()),
            two_sky_scope: RefCell::new(HashMap::new()),
            d_sky_scope: RefCell::new(HashMap::new()),
            k_sky_scope: RefCell::new(HashMap::new()),
        }
    }
}

#[derive(Debug)]
pub struct Object {
    id: ObjectId,
    attr: Vec<f32>,
    dist: f32, // distance from Node I
}

#[derive(Debug)]
pub struct Range {
    start: f32,
    end: f32,
}

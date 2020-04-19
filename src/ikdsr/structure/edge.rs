use crate::prelude::*;
use std::collections::{BTreeMap, HashMap};

pub struct Edge {
    id: EdgeId,
    pub len: f32,
    result: BTreeResult,
}

impl Edge {
    pub fn new(id: EdgeId, len: f32) -> Self {
        Edge {
            id,
            len,
            result: BTreeResult::new(),
        }
    }
}

struct BTreeResult {
    inner: HashMap<QueryId, BTreeMap<f32, ObjectId>>,
}

impl BTreeResult {
    pub fn new() -> Self {
        BTreeResult {
            inner: HashMap::new(),
        }
    }
}

use crate::prelude::*;
use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;

#[derive(Clone)]
pub struct Edge {
    pub id: EdgeId,
    pub len: f32,
    pub objects: Vec<Arc<DataObject>>,
    result: BTreeResult,
}

impl Edge {
    pub fn new(id: EdgeId, len: f32) -> Self {
        Edge {
            id,
            len,
            objects: Vec::new(),
            result: BTreeResult::new(),
        }
    }

    pub fn add_object(&mut self, object: Arc<DataObject>) {
        self.objects.push(object);
    }
}

#[derive(Clone)]
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

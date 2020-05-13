use crate::prelude::*;
use std::collections::{BTreeMap, HashMap};

pub struct Result {
    inner: HashMap<EdgeId, EdgeResult>,
}

impl Result {
    pub fn from_edge_ids(edge_ids: Vec<EdgeId>) -> Self {
        let mut result = Result {
            inner: HashMap::new(),
        };

        for e in edge_ids {
            result.inner.insert(e, EdgeResult::default());
        }

        result
    }

    pub fn insert(&mut self, k: K, edge_id: EdgeId, ranges: Vec<Range>) {
        // TODO: implement using ordered-float
        // https://docs.rs/ordered-float/1.0.2/ordered_float/
    }
}

struct EdgeResult {
    inner: HashMap<K, BTreeMap<f32, Vec<ObjectId>>>,
}

impl Default for EdgeResult {
    fn default() -> Self {
        EdgeResult {
            inner: HashMap::new(),
        }
    }
}

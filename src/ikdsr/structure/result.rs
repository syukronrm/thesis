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

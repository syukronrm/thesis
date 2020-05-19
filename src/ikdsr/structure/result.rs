use crate::prelude::*;
use ordered_float::OrderedFloat;
use std::collections::{BTreeMap, HashMap};

pub struct ResultVoronoi {
    inner: HashMap<EdgeId, EdgeResult>,
}

impl ResultVoronoi {
    pub fn from_edge_ids(edge_ids: Vec<EdgeId>) -> Self {
        let mut result = ResultVoronoi {
            inner: HashMap::new(),
        };

        for e in edge_ids {
            result.inner.insert(e, EdgeResult::default());
        }

        result
    }

    // TODO: DONE implement using ordered-float
    pub fn insert(&mut self, k: K, edge_id: EdgeId, ranges: Vec<Range>) {
        let last_k = Some(k);
        let btree = BTreeMap::new();


        // TODO: implement insertion to result
        for range in &ranges {
        }


        let mut inner = HashMap::new();
        inner.insert(k, btree);
        let edge_result = EdgeResult {
            ranges,
            last_k,
            inner,
        };
    }
}

struct EdgeResult {
    ranges: Vec<Range>,
    last_k: Option<K>,
    inner: HashMap<K, BTreeMap<OrderedFloat<f32>, Vec<ObjectId>>>,
}

impl Default for EdgeResult {
    fn default() -> Self {
        EdgeResult {
            ranges: Vec::new(),
            last_k: None,
            inner: HashMap::new(),
        }
    }
}

use crate::prelude::*;
use std::collections::{BTreeMap, HashMap};

pub struct Edge {
    id: EdgeId,
    pub len: f32,
    result: BTreeResult,
}

struct BTreeResult {
    inner: HashMap<QueryId, BTreeMap<f32, ObjectId>>,
}

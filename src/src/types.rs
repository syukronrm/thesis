use std::sync::Arc;

use crate::prelude::*;

/// Raw node data from dataset
#[derive(Debug)]
pub struct DataNode {
    pub id: NodeId,
    pub lng: f32,
    pub lat: f32,
}

/// Raw edge data from dataset
#[derive(Debug)]
pub struct DataEdge {
    pub id: EdgeId,
    pub ni: Arc<DataNode>,
    pub nj: Arc<DataNode>,
    pub len: f32,
}

impl DataEdge {
    /// Create new raw edge
    pub fn new(id: EdgeId, ni: Arc<DataNode>, nj: Arc<DataNode>) -> DataEdge {
        let diff_lng = ni.lng - nj.lng;
        let diff_lat = ni.lat - nj.lat;
        let len = (diff_lng * diff_lng + diff_lat * diff_lat).sqrt();
        DataEdge { id, ni, nj, len }
    }
}

/// Action for new object
#[derive(Debug)]
pub enum Action {
    Insertion,
    Deletion,
}

/// Raw object data from dataset
pub struct DataObject {
    pub id: ObjectId,
    pub attr: Vec<f32>,
    pub dist: f32, // distance from Node I
    pub edge_id: EdgeId,
    pub action: Action,
}

/// Raw query data from dataset
pub struct Query {
    pub k: K,
    pub dimensions: Vec<DimensionIndex>,
}

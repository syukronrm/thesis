use std::rc::Rc;

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
    pub ni: Rc<DataNode>,
    pub nj: Rc<DataNode>,
    pub len: f32,
}

impl DataEdge {
    /// Create new raw edge
    fn new(id: EdgeId, ni: Rc<DataNode>, nj: Rc<DataNode>) -> DataEdge {
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
    id: ObjectId,
    attr: Vec<f32>,
    dist: f32, // distance from Node I
    edge_id: EdgeId,
    action: Action,
}

use std::rc::Rc;

use crate::prelude::*;

#[derive(Debug)]
pub struct DataNode {
    pub id: NodeId,
    pub lng: f32,
    pub lat: f32,
}

#[derive(Debug)]
pub struct DataEdge {
    pub id: EdgeId,
    pub ni: Rc<DataNode>,
    pub nj: Rc<DataNode>,
    pub len: f32,
}

impl DataEdge {
    fn new(id: EdgeId, ni: Rc<DataNode>, nj: Rc<DataNode>) -> DataEdge {
        let diff_lng = ni.lng - nj.lng;
        let diff_lat = ni.lat - nj.lat;
        let len = (diff_lng * diff_lng + diff_lat * diff_lat).sqrt();
        DataEdge { id, ni, nj, len }
    }
}

#[derive(Debug)]
pub enum Action {
    Insertion,
    Deletion,
}

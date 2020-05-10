use crate::prelude::*;
use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Edge {
    pub id: EdgeId,
    pub len: f32,
    pub ni: NodeId,
    pub nj: NodeId,
    pub objects: Vec<Arc<DataObject>>,
    result: BTreeResult,
}

impl Edge {
    pub fn new(id: EdgeId, len: f32, ni: NodeId, nj: NodeId) -> Self {
        Edge {
            id,
            len,
            ni,
            nj,
            objects: Vec::new(),
            result: BTreeResult::new(),
        }
    }

    pub fn add_object(&mut self, object: Arc<DataObject>) {
        self.objects.push(object);
    }

    #[allow(dead_code)]
    pub fn add_objects(&mut self, objects: Vec<Arc<DataObject>>) {
        for o in objects {
            self.objects.push(o);
        }
    }

    #[allow(dead_code)]
    pub fn objects_in_between(&self, dist_a: f32, dist_b: f32) -> Vec<Arc<DataObject>> {
        self.objects
            .iter()
            .filter_map(|o| {
                if o.dist >= dist_a && o.dist < dist_b {
                    let mut o_new = o.clone();
                    Arc::make_mut(&mut o_new).dist = (o.dist - dist_a) / (dist_b - dist_a);
                    Some(o_new)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn as_data_edge(&self) -> DataEdge {
        DataEdge {
            id: self.id,
            ni: self.ni,
            nj: self.nj,
            len: self.len,
        }
    }
}

#[derive(Clone, Debug)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn edge_objet_in_between() {
        fn new_object(dist: f32) -> Arc<DataObject> {
            Arc::new(DataObject {
                id: 1,
                attr: Vec::new(),
                dist,
                edge_id: 1,
                action: Action::Insertion,
            })
        }

        let mut e = Edge::new(1, 10.0, 1, 1);
        e.add_object(new_object(1.0));
        e.add_object(new_object(2.0));
        e.add_object(new_object(3.0));
        e.add_object(new_object(4.0));
        e.add_object(new_object(5.0));

        let objects = e.objects_in_between(1.5, 3.5);
        assert_eq!(objects.len(), 2);

        let objects = e.objects_in_between(1.5, 1.7);
        assert_eq!(objects.len(), 0);
    }
}

use super::bfs_minheap::BfsMinHeap;
use crate::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone)]
struct Voronoi {}

impl Voronoi {}

struct DomTraverse {
    max_dist: f32,
    originator: Arc<DataObject>,
    pub dominated_by: HashMap<ObjectId, K>,
    pub dominate: HashMap<ObjectId, K>,
}

impl DomTraverse {
    pub fn dominate_dominated_by(graph: &Graph, max_dist: f32, originator: Arc<DataObject>) -> Self {
        let node_index = graph.node_index(originator.id);
        let bfs = BfsMinHeap::new(graph, node_index);

        let mut dominated_by = HashMap::new();
        let mut dominate = HashMap::new();

        for TraverseState { edge_index, .. } in bfs {
            let objects = graph.objects(edge_index);
            for object in objects {
                let mut src_score = 0;
                let mut dst_score = 0;
                for (i, src_val) in originator.attr.iter().enumerate() {
                    let dst_val = object.attr.get(i).unwrap();
                    if src_val > dst_val {
                        src_score += 1;
                    } else if src_val < dst_val {
                        dst_score += 1;
                    } else {
                        src_score += 1;
                        dst_score += 1;
                    }
                }

                if src_score > dst_score {
                    let k = dst_score + 1;
                    dominate.insert(originator.id, k);
                } else if src_score < dst_score {
                    let k = src_score + 1;
                    dominated_by.insert(object.id, k);
                }
            }
        }

        DomTraverse {
            max_dist,
            originator,
            dominated_by,
            dominate,
        }
    }
}

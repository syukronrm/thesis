use super::bfs_minheap::BfsMinHeap;
use crate::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone)]
struct Voronoi {}

impl Voronoi {}

#[derive(Debug)]
struct DomTraverse {
    originator: Arc<DataObject>,
    pub dominated_by: HashMap<ObjectId, K>,
    pub dominate: HashMap<ObjectId, K>,
}

impl DomTraverse {
    /// Get objects dominate and dominated by originator.
    #[allow(dead_code)]
    pub fn dominate_dominated_by(graph: &mut Graph, originator: Arc<DataObject>) -> Self {
        let centroid_id = graph.convert_object_as_node(originator.clone());
        let bfs = BfsMinHeap::new(graph, centroid_id);

        let mut dominated_by = HashMap::new();
        let mut dominate = HashMap::new();

        for TraverseState {
            node_id,
            prev_node_id,
            ..
        } in bfs
        {
            let objects = graph.objects(node_id, prev_node_id);
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
                    dominate.insert(object.id, k);
                } else if src_score < dst_score {
                    let k = src_score + 1;
                    dominated_by.insert(object.id, k);
                }
            }
        }

        graph.remove_node(centroid_id);

        DomTraverse {
            originator,
            dominated_by,
            dominate,
        }
    }

    pub fn dominate_dominated_by_from_id(graph: &mut Graph, object_id: ObjectId) -> Self {
        let object = graph.object(object_id);
        Self::dominate_dominated_by(graph, object)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn compute() {
        let conf = Arc::new(AppConfig::default());
        let mut graph = Graph::new(conf);
        let object_id = 3;
        let result = DomTraverse::dominate_dominated_by_from_id(&mut graph, object_id);
        assert_eq!(result.dominate.get(&2).unwrap(), &3);
        assert_eq!(result.dominated_by.get(&1).unwrap(), &2);

        println!("{:#?}", result);
    }
}

use super::bfs_minheap::BfsMinHeap;
use super::voronoi_minheap::{TraverseState as State, VoronoiMinHeap};
use crate::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone)]
struct Voronoi {
    node_map: HashMap<NodeId, Vec<f32>>,
    voronoi: HashMap<EdgeId, Vec<Range>>,
}

impl Voronoi {
    pub fn initial_voronoi(graph: &mut Graph, object: Arc<DataObject>) -> Self {
        let max_distance = graph.config.max_dist * 2.0;
        let dom_traverse = DomTraverse::dominate_dominated_by_from_id(graph, object.id);
        let dominated_by_objects = dom_traverse.dominated_by_objects();
        let dominated_by_vec: Vec<ObjectId> = dominated_by_objects.keys().map(|k| *k).collect();
        let min_heap = VoronoiMinHeap::new(graph, dominated_by_vec);

        let mut voronoi = Self {
            node_map: HashMap::new(),
            voronoi: HashMap::new(),
        };

        for state in min_heap {
            let State {
                cost_ct_to_ns,
                cost_ct_to_ne,
                cost_pt_to_ne,
                centroid_ct_in_ns,
                centroid_pt_in_ne,
                start_node_id,
                end_node_id,
                edge,
            } = state;
            let edge = edge.unwrap();
            if centroid_pt_in_ne == 0 || centroid_ct_in_ns == centroid_pt_in_ne {
                if cost_ct_to_ne > max_distance {
                    let start = if edge.ni == start_node_id {
                        0.0
                    } else {
                        edge.len
                    };
                    let end = if edge.ni == start_node_id {
                        max_distance - cost_ct_to_ns
                    } else {
                        edge.len - (max_distance - cost_ct_to_ns)
                    };

                    let range = Range {
                        start: start.max(end),
                        end: start.min(end),
                        centroid_id: centroid_ct_in_ns,
                    };
                    voronoi.add_range(range, edge.id);
                } else {
                    let range = Range {
                        start: 0.0,
                        end: edge.len,
                        centroid_id: centroid_ct_in_ns,
                    };
                    voronoi.add_range(range, edge.id);
                }
            } else {
                let center_dist =
                    ((cost_ct_to_ns + cost_pt_to_ne + edge.len) / 2.0) - cost_ct_to_ns;
                if edge.ni == start_node_id {
                    let range = Range {
                        start: 0.0,
                        end: center_dist,
                        centroid_id: centroid_ct_in_ns,
                    };
                    voronoi.add_range(range, edge.id);

                    let range = Range {
                        start: center_dist,
                        end: edge.len,
                        centroid_id: centroid_pt_in_ne,
                    };
                    voronoi.add_range(range, edge.id);
                } else {
                    let center_dist = edge.len - center_dist;
                    let range = Range {
                        start: center_dist,
                        end: edge.len,
                        centroid_id: centroid_ct_in_ns,
                    };
                    voronoi.add_range(range, edge.id);

                    let range = Range {
                        start: 0.0,
                        end: center_dist,
                        centroid_id: centroid_pt_in_ne,
                    };
                    voronoi.add_range(range, edge.id);
                }
            }
        }

        voronoi
    }

    pub fn add_range(&mut self, range: Range, edge_id: EdgeId) {
        if let Some(ranges) = self.voronoi.get_mut(&edge_id) {
            ranges.push(range);
        } else {
            self.voronoi.insert(edge_id, vec![range]);
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Range {
    pub start: f32,
    pub end: f32,
    pub centroid_id: CentroidId,
}

#[derive(Debug)]
struct DomTraverse {
    originator: Arc<DataObject>,
    pub dominated_by: HashMap<K, Vec<ObjectId>>,
    pub dominate: HashMap<K, Vec<ObjectId>>,
}

impl DomTraverse {
    /// Get objects dominate and dominated by originator.
    #[allow(dead_code)]
    fn dominate_dominated_by(graph: &mut Graph, originator: Arc<DataObject>) -> Self {
        let centroid_id = graph.convert_object_as_node(originator.clone());
        let bfs = BfsMinHeap::new(graph, centroid_id);

        let mut dominated_by: HashMap<K, Vec<ObjectId>> = HashMap::new();
        let mut dominate: HashMap<K, Vec<ObjectId>> = HashMap::new();

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
                    if let Some(a) = dominate.get_mut(&k) {
                        a.push(object.id);
                    } else {
                        dominate.insert(k, vec![object.id]);
                    }
                } else if src_score < dst_score {
                    let k = src_score + 1;
                    if let Some(a) = dominated_by.get_mut(&k) {
                        a.push(object.id);
                    } else {
                        dominated_by.insert(k, vec![object.id]);
                    }
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

    fn dominate_dominated_by_from_id(graph: &mut Graph, object_id: ObjectId) -> Self {
        let object = graph.object(object_id);
        Self::dominate_dominated_by(graph, object)
    }

    fn dominated_by_objects(&self) -> HashMap<ObjectId, bool> {
        let mut object_ids = HashMap::new();
        for (_, vec_obj_id) in &self.dominated_by {
            for obj_id in vec_obj_id {
                object_ids.insert(*obj_id, true);
            }
        }
        object_ids
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
        assert_eq!(result.dominate.get(&3).unwrap().len(), 1);
        assert_eq!(result.dominated_by.get(&2).unwrap().len(), 1);

        println!("{:#?}", result);
    }
}

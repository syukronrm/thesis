use super::bfs_minheap::BfsMinHeap;
use super::voronoi_minheap::{TraverseState as State, VoronoiMinHeap};
use crate::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Voronoi {
    scope: HashMap<EdgeId, Vec<Range>>,
}

impl Voronoi {
    pub fn initial_voronoi(graph: &mut Graph, object_id: ObjectId) -> Self {
        let max_distance = graph.config.max_dist * 2.0;
        let dom_traverse = DomTraverse::dominate_dominated_by_from_id(graph, object_id);
        let mut dominated_by_vec = dom_traverse.dominated_by_objects();
        dominated_by_vec.push(object_id);
        let centroid_ids = graph.convert_object_ids_to_node(dominated_by_vec);
        let mut map_objects_k = dom_traverse.map_dominated_by_objects_k();
        map_objects_k.insert(object_id, graph.config.max_dim);
        let mut min_heap = VoronoiMinHeap::new(graph, centroid_ids, map_objects_k);

        let mut voronoi = Self {
            scope: HashMap::new(),
        };

        for state in min_heap.by_ref() {
            let State {
                cost_ct_to_ns,
                cost_ct_to_ne,
                cost_pt_to_ne,
                centroid_ct_in_ns,
                centroid_pt_in_ne,
                start_node_id,
                end_node_id: _,
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
                    voronoi.add_scope(range, edge.id);
                } else {
                    let range = Range {
                        start: 0.0,
                        end: edge.len,
                        centroid_id: centroid_ct_in_ns,
                    };
                    voronoi.add_scope(range, edge.id);
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
                    voronoi.add_scope(range, edge.id);

                    let range = Range {
                        start: center_dist,
                        end: edge.len,
                        centroid_id: centroid_pt_in_ne,
                    };
                    voronoi.add_scope(range, edge.id);
                } else {
                    let c = edge.len - center_dist;
                    let range = Range {
                        start: c,
                        end: edge.len,
                        centroid_id: centroid_ct_in_ns,
                    };
                    voronoi.add_scope(range, edge.id);

                    let range = Range {
                        start: 0.0,
                        end: c,
                        centroid_id: centroid_pt_in_ne,
                    };
                    voronoi.add_scope(range, edge.id);
                }
            }
        }

        voronoi.convert_voronoi_to_original_edge(graph.map_new_edge());
        voronoi
    }

    fn add_scope(&mut self, range: Range, edge_id: EdgeId) {
        if let Some(ranges) = self.scope.get_mut(&edge_id) {
            ranges.push(range);
        } else {
            self.scope.insert(edge_id, vec![range]);
        }
    }

    fn convert_voronoi_to_original_edge(&mut self, map_new_edge: HashMap<EdgeId, Vec<EdgeId>>) {
        for (edge_id, vec_new_edge_id) in map_new_edge {
            let mut adjusted_scopes: HashMap<CentroidId, Range> = HashMap::new();
            let mut start_range = 0.0;
            for new_edge_id in vec_new_edge_id {
                let mut scopes = self.scope.get(&new_edge_id).unwrap().clone();
                scopes.sort_by(|a, b| a.start.partial_cmp(&b.start).unwrap());
                for scope in scopes {
                    if let Some(s) = adjusted_scopes.get_mut(&scope.centroid_id) {
                        s.end = s.end + scope.end;
                        start_range = s.end;
                    } else {
                        let new_scope = Range {
                            start: start_range,
                            end: start_range + (scope.end - scope.start),
                            centroid_id: scope.centroid_id,
                        };
                        start_range = new_scope.end;
                        adjusted_scopes.insert(scope.centroid_id, new_scope);
                    }
                }
                self.scope.remove(&new_edge_id);
            }
            let ranges: Vec<Range> = adjusted_scopes.values().map(|v| v.clone()).collect();
            self.scope.remove(&edge_id);
            self.scope.insert(edge_id, ranges);
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

    fn dominated_by_objects(&self) -> Vec<ObjectId> {
        let mut object_ids = Vec::new();
        for (_, vec_obj_id) in &self.dominated_by {
            for obj_id in vec_obj_id {
                object_ids.push(*obj_id);
            }
        }
        object_ids
    }

    fn map_dominated_by_objects_k(&self) -> HashMap<ObjectId, K> {
        let mut map_objects_k = HashMap::new();
        for (k, vec_object_id) in &self.dominated_by {
            for obj_id in vec_object_id {
                map_objects_k.insert(*obj_id, *k);
            }
        }
        map_objects_k
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn dom_traverse_test() {
        let conf = Arc::new(AppConfig::default());
        let mut graph = Graph::new(conf);
        let object_id = 3;
        let result = DomTraverse::dominate_dominated_by_from_id(&mut graph, object_id);
        assert_eq!(result.dominate.get(&3).unwrap().len(), 1);
        assert_eq!(result.dominated_by.get(&2).unwrap().len(), 1);

        println!("{:#?}", result);
    }

    #[test]
    fn voronoi_test() {
        let conf = Arc::new(AppConfig::default());
        let mut graph = Graph::new(conf);
        let object_id = 2;
        let voronoi = Voronoi::initial_voronoi(&mut graph, object_id);
        println!("{:#?}", voronoi);

        let tests = [(4, 1), (2, 1), (1, 1), (5, 2), (3, 2)];
        for (edge_id, range_len) in tests.iter() {
            let mut is_exist = false;
            for (e, ranges) in &voronoi.scope {
                if e == edge_id && ranges.len() == *range_len {
                    is_exist = true;
                }
            }
            assert!(is_exist);
        }
    }
}

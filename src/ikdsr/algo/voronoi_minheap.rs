use crate::prelude::*;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::sync::Arc;

type CentroidId = NodeId;

struct VoronoiMinHeap<'a> {
    pub graph: &'a Graph,
    pub max_dist: f32,
    pub min_heap: BinaryHeap<TraverseState>,
    pub cost_map: HashMap<NodeId, (NodeId, f32)>,
    visited: HashMap<EdgeId, bool>, // key = node_id, value = centroid
}

impl<'a> VoronoiMinHeap<'a> {
    pub fn new(graph: &'a mut Graph, centroid_ids: Vec<CentroidId>) -> Self {
        let mut min_heap = BinaryHeap::new();
        for centroid_id in centroid_ids {
            min_heap.push(TraverseState {
                cost_ct_to_ns: 0.0,
                cost_ct_to_ne: 0.0,
                cost_pt_to_ne: 0.0,
                centroid_ct_in_ns: centroid_id,
                centroid_pt_in_ne: centroid_id,
                start_node_id: centroid_id,
                end_node_id: centroid_id,
            });
        }

        VoronoiMinHeap {
            graph,
            max_dist: graph.config.max_dist,
            min_heap,
            cost_map: HashMap::new(),
            visited: HashMap::new(),
        }
    }

    pub fn from_objects(graph: &'a mut Graph, centroids: Vec<Arc<DataObject>>) -> Self {
        let centroid_ids = graph.convert_objects_to_node(centroids);
        Self::new(graph, centroid_ids)
    }

    /// Return true if already visited, if not visit it and return false.
    fn visit(&mut self, a: NodeId, b: NodeId) -> bool {
        if let Some(edge_id) = self.graph.edge_id(a, b) {
            if self.visited.get(&edge_id).is_some() {
                return true;
            } else {
                self.visited.insert(edge_id, true);
            }
        }
        return false;
    }

    /// Return true is already visited.
    fn is_visited(&self, a: NodeId, b: NodeId) -> bool {
        if let Some(edge_id) = self.graph.edge_id(a, b) {
            if self.visited.get(&edge_id).is_some() {
                return true;
            }
        }
        false
    }
}

impl<'a> Iterator for VoronoiMinHeap<'a> {
    type Item = TraverseState;

    fn next(&mut self) -> Option<Self::Item> {
        let mut returned_state = None;
        while let Some(state) = self.min_heap.pop() {
            let TraverseState {
                cost_ct_to_ns,
                cost_ct_to_ne,
                cost_pt_to_ne: _,
                centroid_ct_in_ns,
                centroid_pt_in_ne: _,
                start_node_id,
                end_node_id,
            } = state;

            if cost_ct_to_ns > self.max_dist * 2.0 {
                continue;
            }

            if self.visit(start_node_id, end_node_id) {
                continue;
            }

            for node_id in self.graph.neighbors(end_node_id) {
                if self.is_visited(node_id, end_node_id) {
                    continue;
                }

                let cost_next = cost_ct_to_ne + self.graph.edge_len(node_id, end_node_id);
                let some_cost = self.cost_map.get_mut(&node_id);
                if let Some(struct_cost) = some_cost {
                    let (existing_centroid, prev_cost) = struct_cost.clone();
                    if (existing_centroid == centroid_ct_in_ns && cost_next < prev_cost) || (existing_centroid != centroid_ct_in_ns) {
                            *struct_cost = (centroid_ct_in_ns, cost_next);
                            self.min_heap.push(TraverseState {
                                cost_ct_to_ns: cost_ct_to_ne,
                                cost_ct_to_ne: cost_next,
                                cost_pt_to_ne: prev_cost,
                                centroid_ct_in_ns: centroid_ct_in_ns,
                                centroid_pt_in_ne: existing_centroid,
                                start_node_id: end_node_id,
                                end_node_id: node_id,
                            });
                    }
                } else {
                    self.cost_map
                        .insert(node_id, (centroid_ct_in_ns, cost_next));
                    self.min_heap.push(TraverseState {
                        cost_ct_to_ns: cost_ct_to_ne,
                        cost_ct_to_ne: cost_next,
                        cost_pt_to_ne: 0.0,
                        centroid_ct_in_ns: centroid_ct_in_ns,
                        centroid_pt_in_ne: 0,
                        start_node_id: end_node_id,
                        end_node_id: node_id,
                    });
                }
            }

            // ifgnore if start from centroid
            if state.start_node_id != state.end_node_id {
                returned_state = Some(state);
                break;
            }
        }

        returned_state
    }
}

#[derive(Copy, Clone, Debug)]
pub struct TraverseState {
    pub cost_ct_to_ns: f32,            // cost of current traverse to node start
    pub cost_ct_to_ne: f32,            // cost of current traverse to node end
    pub cost_pt_to_ne: f32,            // cost of previous traverse to node end
    pub centroid_ct_in_ns: CentroidId, // centroid of current traverse in node start
    pub centroid_pt_in_ne: CentroidId, // centroid of previous traverse in node end
    pub start_node_id: NodeId,         // node start
    pub end_node_id: NodeId,           // node end
}

impl Ord for TraverseState {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.cost_ct_to_ne.is_nan() || other.cost_ct_to_ne.is_nan() {
            panic!("TraverseState.cost shouldn't be a NaN!");
        }

        if self.cost_ct_to_ne < other.cost_ct_to_ne {
            Ordering::Less
        } else if self.cost_ct_to_ne > other.cost_ct_to_ne {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for TraverseState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.cost_ct_to_ne.partial_cmp(&self.cost_ct_to_ne)
    }
}

impl PartialEq for TraverseState {
    fn eq(&self, other: &Self) -> bool {
        if self.cost_ct_to_ne.is_nan() || other.cost_ct_to_ne.is_nan() {
            panic!("State.cost is NaN!");
        }
        self.cost_ct_to_ne == other.cost_ct_to_ne
    }
}

impl Eq for TraverseState {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_voronoi_minheap() {
        let conf = Arc::new(AppConfig::default());
        let mut graph = Graph::new(conf);
        let voronoi_minheap = VoronoiMinHeap::new(&mut graph, vec![2, 5]);

        let mut count = 0;
        for state in voronoi_minheap {
            count += 1;
            println!("{:#?}", state);
        }

        assert_eq!(count, 5);
    }
}

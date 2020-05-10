use crate::prelude::*;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::sync::Arc;

pub struct VoronoiMinHeap<'a> {
    graph: &'a Graph,
    pub max_dist: f32,
    pub min_heap: BinaryHeap<TraverseState>,
    pub cost_map: HashMap<NodeId, (CentroidId, f32)>,
    visited: HashMap<EdgeId, bool>,
}

impl<'a> VoronoiMinHeap<'a> {
    pub fn new(graph: &'a mut Graph, centroid_ids: Vec<CentroidId>) -> Self {
        let mut min_heap = BinaryHeap::new();
        let mut cost_map = HashMap::new();
        for centroid_id in centroid_ids {
            for node_id in graph.neighbors(centroid_id) {
                let edge = graph.edge(node_id, centroid_id).unwrap();
                min_heap.push(TraverseState {
                    cost_ct_to_ns: 0.0,
                    cost_ct_to_ne: edge.len,
                    cost_pt_to_ne: 0.0,
                    centroid_ct_in_ns: centroid_id,
                    centroid_pt_in_ne: 0,
                    start_node_id: centroid_id,
                    end_node_id: node_id,
                    edge: SimpleEdge::from_some(Some(edge)),
                });

                if let Some((_cen, cost)) = cost_map.get(&node_id) {
                    if *cost > edge.len {
                        cost_map.insert(node_id, (centroid_id, edge.len));
                    }
                } else {
                    cost_map.insert(node_id, (centroid_id, edge.len));
                }
                cost_map.insert(centroid_id, (centroid_id, 0.0));
            }
        }

        VoronoiMinHeap {
            graph,
            max_dist: graph.config.max_dist,
            min_heap,
            cost_map,
            visited: HashMap::new(),
        }
    }

    pub fn from_objects(graph: &'a mut Graph, centroids: Vec<Arc<DataObject>>) -> Self {
        graph.convert_objects_to_node(centroids);
        let mut centroid_ids = Vec::new();
        for (_edge_id, mut node_ids) in graph.map_new_node() {
            centroid_ids.append(&mut node_ids);
        }
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

    pub fn map_new_edge(&self) -> HashMap<EdgeId, Vec<NodeId>> {
        self.graph.map_new_edge()
    }
}

impl<'a> Iterator for VoronoiMinHeap<'a> {
    type Item = TraverseState;

    fn next(&mut self) -> Option<Self::Item> {
        let mut returned_state = None;
        while let Some(mut state) = self.min_heap.pop() {
            let TraverseState {
                cost_ct_to_ns,
                cost_ct_to_ne,
                cost_pt_to_ne: _,
                centroid_ct_in_ns,
                centroid_pt_in_ne: _,
                start_node_id,
                end_node_id,
                edge: _,
            } = state;

            if let Some((centroid_id, cost)) = self.cost_map.get(&state.end_node_id) {
                state.centroid_pt_in_ne = *centroid_id;
                state.cost_pt_to_ne = *cost;
            }

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

                let edge = self.graph.edge(node_id, end_node_id);
                let cost_next = {
                    if let Some(edge) = edge {
                        cost_ct_to_ne + edge.len
                    } else {
                        cost_ct_to_ne
                    }
                };
                let some_cost = self.cost_map.get_mut(&node_id);
                if let Some(struct_cost) = some_cost {
                    let (existing_centroid, prev_cost) = struct_cost.clone();
                    if (existing_centroid == centroid_ct_in_ns && cost_next < prev_cost)
                        || (existing_centroid != centroid_ct_in_ns)
                    {
                        if cost_next < prev_cost {
                            *struct_cost = (centroid_ct_in_ns, cost_next);
                        }
                        self.min_heap.push(TraverseState {
                            cost_ct_to_ns: cost_ct_to_ne,
                            cost_ct_to_ne: cost_next,
                            cost_pt_to_ne: prev_cost,
                            centroid_ct_in_ns: centroid_ct_in_ns,
                            centroid_pt_in_ne: existing_centroid,
                            start_node_id: end_node_id,
                            end_node_id: node_id,
                            edge: SimpleEdge::from_some(edge),
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
                        edge: SimpleEdge::from_some(edge),
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

#[derive(Clone, Debug)]
pub struct TraverseState {
    pub cost_ct_to_ns: f32,            // cost of current traverse to node start
    pub cost_ct_to_ne: f32,            // cost of current traverse to node end
    pub cost_pt_to_ne: f32,            // cost of previous traverse to node end
    pub centroid_ct_in_ns: CentroidId, // centroid of current traverse in node start
    pub centroid_pt_in_ne: CentroidId, // centroid of previous traverse in node end
    pub start_node_id: NodeId,         // node start
    pub end_node_id: NodeId,           // node end
    pub edge: Option<SimpleEdge>,
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

#[derive(Copy, Clone, Debug)]
pub struct SimpleEdge {
    pub id: EdgeId,
    pub ni: NodeId,
    pub nj: NodeId,
    pub len: f32,
}

impl SimpleEdge {
    pub fn from_some(edge: Option<&Edge>) -> Option<Self> {
        if let Some(e) = edge {
            Some(SimpleEdge {
                id: e.id,
                ni: e.ni,
                nj: e.nj,
                len: e.len,
            })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_voronoi_minheap() {
        let conf = Arc::new(AppConfig::default());
        let mut graph = Graph::new(conf);
        let voronoi_minheap = VoronoiMinHeap::new(&mut graph, vec![1, 2, 3]);

        let mut count = 0;
        for state in voronoi_minheap {
            count += 1;
            println!("{:#?}", state);
        }

        assert_eq!(count, 5);
    }
}

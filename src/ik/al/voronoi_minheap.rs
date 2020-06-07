use crate::prelude::*;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

// TODO: DONE save k value
pub struct VoronoiMinHeap<'a> {
    graph: &'a Graph,
    max_dist: f32,
    min_heap: BinaryHeap<TraverseState>,
    cost_map: HashMap<NodeId, (CentroidId, f32)>,
    visited: HashMap<EdgeId, bool>,
    map_object_id_k: HashMap<ObjectId, K>,
    map_centroid_edge_id: HashMap<EdgeId, (CentroidId, K)>,
    min_heap_reserve: Vec<TraverseState>,
    is_initial: bool,
    pub current_k: K,
}

impl<'a> VoronoiMinHeap<'a> {
    pub fn new(
        graph: &'a mut Graph,
        centroid_ids: Vec<CentroidId>,
        map_object_id_k: HashMap<ObjectId, K>,
        start_k: K,
    ) -> Self {
        let mut min_heap = BinaryHeap::new();
        let mut cost_map = HashMap::new();
        for centroid_id in centroid_ids {
            for node_id in graph.neighbors(centroid_id) {
                let edge = graph.edge(node_id, centroid_id).unwrap();
                let smallest_k = *map_object_id_k
                    .get(&Graph::as_object_id(centroid_id))
                    .unwrap();
                min_heap.push(TraverseState {
                    cost_ct_to_ns: 0.0,
                    cost_ct_to_ne: edge.len,
                    cost_pt_to_ne: 0.0,
                    centroid_ct_in_ns: centroid_id,
                    centroid_pt_in_ne: 0,
                    start_node_id: centroid_id,
                    end_node_id: node_id,
                    smallest_k: (smallest_k, Position::Start),
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
            map_object_id_k,
            map_centroid_edge_id: HashMap::new(),
            min_heap_reserve: Vec::new(),
            is_initial: true,
            current_k: start_k,
        }
    }

    /// Pop min_heap_reverse to min_heap
    #[allow(dead_code)]
    pub fn pop_min_heap_reserve(&mut self) {
        let mut min_heap: BinaryHeap<TraverseState> = BinaryHeap::new();
        let min_heap_reserve: Vec<TraverseState> = self
            .min_heap_reserve
            .iter()
            .filter(|t| {
                if t.smallest_k.0 <= self.current_k {
                    min_heap.push(**t);
                }

                self.current_k > t.smallest_k.0
            })
            .map(|t| *t)
            .collect();

        self.min_heap_reserve = min_heap_reserve;
        self.min_heap = min_heap
            .iter()
            .map(|t| {
                let mut t = *t;
                match t.smallest_k.1 {
                    Position::Start => {
                        self.remove_cost(t.end_node_id);
                        t.centroid_pt_in_ne = t.centroid_ct_in_ns;
                        t.cost_pt_to_ne = t.cost_ct_to_ne;

                        t
                    }
                    Position::End => {
                        self.remove_cost(t.start_node_id);
                        TraverseState {
                            cost_ct_to_ns: t.cost_pt_to_ne,
                            cost_ct_to_ne: t.cost_pt_to_ne,
                            cost_pt_to_ne: t.cost_pt_to_ne + t.edge.unwrap().len,
                            centroid_ct_in_ns: t.centroid_pt_in_ne,
                            centroid_pt_in_ne: t.centroid_pt_in_ne,
                            start_node_id: t.end_node_id,
                            end_node_id: t.start_node_id,
                            smallest_k: t.smallest_k,
                            edge: t.edge,
                        }
                    }
                }
            })
            .collect();
    }

    // pub fn from_objects(graph: &'a mut Graph, centroids: Vec<Arc<DataObject>>) -> Self {
    //     graph.convert_objects_to_node(centroids);
    //     let mut centroid_ids = Vec::new();
    //     for (_edge_id, mut node_ids) in graph.map_new_node() {
    //         centroid_ids.append(&mut node_ids);
    //     }
    //     Self::new(graph, centroid_ids)
    // }

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

    fn save_map_object_id_to_k(
        &mut self,
        edge: SimpleEdge,
        s: NodeId,
        e: NodeId,
        curr: CentroidId,
        prev: CentroidId,
    ) {
        let curr = Graph::as_object_id(curr);
        let prev = Graph::as_object_id(prev);
        if s != e {
            if prev == 0 {
                let k_curr = self.map_object_id_k.get(&curr).unwrap();
                self.map_centroid_edge_id.insert(edge.id, (curr, *k_curr));
            } else {
                let k_curr = self.map_object_id_k.get(&curr).unwrap();
                let k_prev = self.map_object_id_k.get(&prev).unwrap();

                if k_curr < k_prev {
                    self.map_centroid_edge_id.insert(edge.id, (curr, *k_curr));
                } else {
                    self.map_centroid_edge_id.insert(edge.id, (prev, *k_prev));
                }
            }
        }
    }

    fn reserve_state(&mut self, state: TraverseState) {
        if state.start_node_id == state.end_node_id {
            return;
        }

        if state.centroid_ct_in_ns != state.centroid_pt_in_ne && state.smallest_k.0 > self.current_k
        {
            if state.centroid_pt_in_ne == 0 {
                self.min_heap_reserve.push(state);
            } else if self.k_of_object(state.centroid_ct_in_ns) < self.graph.config.max_dim
                || self.k_of_object(state.centroid_pt_in_ne) < self.graph.config.max_dim
            {
                self.min_heap_reserve.push(state);
            }
        }
    }

    pub fn map_new_edge(&self) -> HashMap<EdgeId, Vec<NodeId>> {
        self.graph.map_new_edge()
    }

    pub fn set_initialized(&mut self) {
        self.is_initial = false;
    }

    pub fn set_k(&mut self, k: K) {
        self.current_k = k;
    }

    pub fn clear_visited(&mut self) {
        self.visited = HashMap::new();
    }

    pub fn is_original_edge(&self, edge_id: EdgeId) -> bool {
        if edge_id > 100000 {
            false
        } else {
            true
        }
    }

    fn k_of_object(&self, object_id: ObjectId) -> K {
        let object_id = Graph::as_object_id(object_id);
        *self.map_object_id_k.get(&object_id).unwrap()
    }

    fn smallest_k(&self, start: ObjectId, end: ObjectId) -> (K, Position) {
        let start_k = self.k_of_object(start);
        let end_k = self.k_of_object(end);
        if start_k < end_k {
            (start_k, Position::Start)
        } else {
            (end_k, Position::End)
        }
    }

    pub fn remove_cost(&mut self, node_id: NodeId) {
        self.cost_map.remove(&node_id);
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Position {
    Start,
    End,
}

// TODO: DONE modify the iterator by considering k value when traversing
// idea: continue traverse when next edge's k is 0
//      or equal to k from input
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
                centroid_pt_in_ne,
                start_node_id,
                end_node_id,
                smallest_k: _,
                edge,
            } = state;

            if let Some((centroid_id, cost)) = self.cost_map.get(&state.end_node_id) {
                state.centroid_pt_in_ne = *centroid_id;
                state.cost_pt_to_ne = *cost;
            }

            if cost_ct_to_ns > self.max_dist {
                continue;
            }

            if self.visit(start_node_id, end_node_id) {
                continue;
            }

            // save centroid with smallest k
            self.save_map_object_id_to_k(
                edge.unwrap(),
                start_node_id,
                end_node_id,
                centroid_ct_in_ns,
                centroid_pt_in_ne,
            );

            self.reserve_state(state);

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

                if !self.is_initial {
                    if let Some(edge) = edge {
                        if let Some((_centroid_id, k)) = self.map_centroid_edge_id.get(&edge.id) {
                            if *k > self.current_k {
                                continue;
                            }
                        }
                    }
                    self.cost_map.remove(&node_id);
                }

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
                            smallest_k: self.smallest_k(centroid_ct_in_ns, existing_centroid),
                            edge: SimpleEdge::from_some(edge),
                        });
                    }
                } else {
                    self.cost_map
                        .insert(node_id, (centroid_ct_in_ns, cost_next));
                    let smallest_k = self.k_of_object(centroid_ct_in_ns);
                    self.min_heap.push(TraverseState {
                        cost_ct_to_ns: cost_ct_to_ne,
                        cost_ct_to_ne: cost_next,
                        cost_pt_to_ne: 0.0,
                        centroid_ct_in_ns: centroid_ct_in_ns,
                        centroid_pt_in_ne: 0,
                        start_node_id: end_node_id,
                        end_node_id: node_id,
                        smallest_k: (smallest_k, Position::Start),
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

#[derive(Copy, Clone, Debug)]
pub struct TraverseState {
    pub cost_ct_to_ns: f32,            // cost of current traverse to node start
    pub cost_ct_to_ne: f32,            // cost of current traverse to node end
    pub cost_pt_to_ne: f32,            // cost of previous traverse to node end
    pub centroid_ct_in_ns: CentroidId, // centroid of current traverse in node start
    pub centroid_pt_in_ne: CentroidId, // centroid of previous traverse in node end
    pub start_node_id: NodeId,         // node start
    pub end_node_id: NodeId,           // node end
    pub smallest_k: (K, Position),
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
    use std::sync::Arc;

    #[test]
    fn new_voronoi_minheap() {
        let conf = Arc::new(AppConfig::default());
        let mut graph = Graph::new(conf);
        let mut map_object_id_k = HashMap::new();
        map_object_id_k.insert(1, 3);
        map_object_id_k.insert(2, 4);
        map_object_id_k.insert(3, 3);
        let voronoi_minheap = VoronoiMinHeap::new(&mut graph, vec![1, 2, 3], map_object_id_k, 3);

        let mut count = 0;
        for state in voronoi_minheap {
            count += 1;
            println!("{:#?}", state);
        }

        assert_eq!(count, 5);
    }
}

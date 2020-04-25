use crate::prelude::*;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fmt;
use std::sync::Arc;

/// Traverse a graph with BFS feat `min_heap`.
#[derive(Clone)]
pub struct BfsMinHeap<'a> {
    graph: &'a Graph,
    max_dist: f32,
    min_heap: BinaryHeap<TraverseState>,
    cost_map: HashMap<NodeId, f32>,
}

impl<'a> BfsMinHeap<'a> {
    /// Initialize new traversal. What it does?
    ///
    /// - Set all node cost as f32::MAX.
    /// - Push neighbors of centroid to `min_heap`.
    pub fn new(graph: &'a Graph, start: NodeId) -> Self {
        let max_dist = graph.config.max_dist;

        let mut cost_map: HashMap<NodeId, f32> =
            graph.nodes().map(|x| (x, std::f32::MAX)).collect();

        *cost_map.get_mut(&start).unwrap() = 0.0;
        let mut min_heap = BinaryHeap::new();
        for node_id in graph.neighbors(start) {
            let cost = graph.edge_len(start, node_id);
            min_heap.push(TraverseState {
                prev_node_id: start,
                node_id: node_id,
                cost,
            });

            // replace cost of `node`
            *cost_map.get_mut(&node_id).unwrap() = cost;
        }

        BfsMinHeap {
            graph,
            max_dist,
            min_heap,
            cost_map,
        }
    }

    // TODO: create traversal using object
    // create object as node
    // returning BfsMinHeap
    #[allow(dead_code, unused_variables)]
    fn from_object(graph: &'a Graph, object_id: Arc<DataObject>) {}
}

impl<'a> Iterator for BfsMinHeap<'a> {
    type Item = TraverseState;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(state) = self.min_heap.pop() {
            let TraverseState {
                node_id: node_id_src,
                cost,
                ..
            } = state;

            for node_id in self.graph.neighbors(node_id_src) {
                let cost_next = cost + self.graph.edge_len(node_id, node_id_src);
                let prev_cost = *self.cost_map.get(&node_id).unwrap();
                if cost_next < prev_cost && cost_next < self.max_dist * 2.0 {
                    self.min_heap.push(TraverseState {
                        prev_node_id: node_id_src,
                        node_id,
                        cost: cost_next,
                    });

                    // replace the cost if `node_index`
                    *self.cost_map.get_mut(&node_id).unwrap() = cost_next;
                }
            }

            Some(state)
        } else {
            None
        }
    }
}

impl<'a> fmt::Debug for BfsMinHeap<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut f_main = f.debug_list();
        for state in self.clone() {
            f_main.entry(&state);
        }
        f_main.finish()
    }
}

/// Save the cost of node and its previous edge.
#[derive(Copy, Clone, Debug)]
pub struct TraverseState {
    pub cost: f32,
    pub node_id: NodeId,
    pub prev_node_id: NodeId,
}

impl Ord for TraverseState {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.cost.is_nan() || other.cost.is_nan() {
            panic!("TraverseState.cost shouldn't be a NaN!");
        }

        if self.cost < other.cost {
            Ordering::Less
        } else if self.cost > other.cost {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for TraverseState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.cost.partial_cmp(&self.cost)
    }
}

impl PartialEq for TraverseState {
    fn eq(&self, other: &Self) -> bool {
        if self.cost.is_nan() || other.cost.is_nan() {
            panic!("State.cost is NaN!");
        }
        self.cost == other.cost
    }
}

impl Eq for TraverseState {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn bfs_min_heap_new() {
        let conf = Arc::new(AppConfig::default());
        let graph = Graph::new(conf);
        let mut bfs = BfsMinHeap::new(&graph, 1);

        let node_id_orders = [2, 3, 4, 6, 5];
        for node_id in node_id_orders.iter() {
            let state = bfs.next().unwrap();
            assert_eq!(state.node_id, *node_id);
        }
    }
}

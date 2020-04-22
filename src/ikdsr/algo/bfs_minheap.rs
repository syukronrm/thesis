use crate::prelude::*;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fmt;

/// Traverse a graph with BFS feat `min_heap`.
#[derive(Clone)]
struct BfsMinHeap {
    graph: Graph,
    max_dist: f32,
    min_heap: BinaryHeap<TraverseState>,
    cost_map: HashMap<NodeIndex, f32>,
}

impl BfsMinHeap {
    /// Initialize new traversal. What it does?
    ///
    /// - Set all node cost as f32::MAX.
    /// - Push neighbors of centroid to `min_heap`.
    pub fn new(graph: Graph, start: NodeIndex) -> Self {
        let max_dist = graph.config.max_dist;

        let mut cost_map: HashMap<NodeIndex, f32> =
            graph.node_indices().map(|x| (x, std::f32::MAX)).collect();

        *cost_map.get_mut(&start).unwrap() = 0.0;
        let mut min_heap = BinaryHeap::new();
        for node_index in graph.neighbors(start) {
            let edge_index = graph.find_edge(start, node_index);
            let cost = graph.edge_len(edge_index);
            min_heap.push(TraverseState {
                edge_index,
                node_index,
                cost,
            });

            // replace cost of `node`
            *cost_map.get_mut(&node_index).unwrap() = cost;
        }

        BfsMinHeap {
            graph,
            max_dist,
            min_heap,
            cost_map,
        }
    }
}

impl Iterator for BfsMinHeap {
    type Item = TraverseState;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(state) = self.min_heap.pop() {
            let TraverseState {
                node_index: node_index_src,
                cost,
                ..
            } = state;

            for node_index in self.graph.neighbors(node_index_src) {
                let edge_index = self.graph.find_edge(node_index, node_index_src);
                let cost_next = self.graph.edge_len(edge_index) + cost;

                let prev_cost = *self.cost_map.get(&node_index).unwrap();
                if cost_next < prev_cost && cost_next < self.max_dist * 2.0 {
                    self.min_heap.push(TraverseState {
                        edge_index,
                        node_index,
                        cost: cost_next,
                    });

                    // replace the cost if `node_index`
                    *self.cost_map.get_mut(&node_index).unwrap() = cost_next;
                }
            }

            Some(state)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct TraverseStateDebug {
    edge_id: EdgeId,
    node_id: NodeId,
    cost: f32,
}

impl fmt::Debug for BfsMinHeap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut f_main = f.debug_list();
        for state in self.clone() {
            let node_id = self.graph.node_id(state.node_index);
            let edge_id = self.graph.edge_id(state.edge_index);
            let state = TraverseStateDebug {
                node_id,
                edge_id,
                cost: state.cost,
            };
            f_main.entry(&state);
        }
        f_main.finish()
    }
}

/// Save the cost of node and its previous edge.
#[derive(Copy, Clone)]
struct TraverseState {
    cost: f32,
    node_index: NodeIndex,
    edge_index: EdgeIndex,
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
        let n1_index = graph.node_index(1);
        let mut bfs = BfsMinHeap::new(graph.clone(), n1_index);

        let node_id_orders = [2, 3, 4, 6, 5];
        for node_id in node_id_orders.iter() {
            let state = bfs.next().unwrap();
            let node_id_traverse = graph.node_id(state.node_index);
            assert_eq!(node_id_traverse, *node_id);
        }
    }
}

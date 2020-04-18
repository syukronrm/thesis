use crate::prelude::*;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

/// Traverse a graph with BFS feat `min_heap`.
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
    type Item = PairNodeEdge;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(state) = self.min_heap.pop() {
            let TraverseState {
                edge_index,
                node_index,
                cost,
            } = state;

            for node_index in self.graph.neighbors(node_index) {
                let edge_index = self.graph.find_edge(node_index, node_index);
                let cost_next = self.graph.edge_len(edge_index) + cost;

                if cost_next < self.max_dist && cost_next < self.max_dist * 2.0 {
                    self.min_heap.push(TraverseState {
                        edge_index,
                        node_index,
                        cost: cost_next,
                    });

                    // replace the cost if `node_index`
                    *self.cost_map.get_mut(&node_index).unwrap() = cost_next;
                }
            }

            Some(PairNodeEdge {
                node_index,
                edge_index,
            })
        } else {
            None
        }
    }
}

struct PairNodeEdge {
    node_index: NodeIndex,
    edge_index: EdgeIndex,
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

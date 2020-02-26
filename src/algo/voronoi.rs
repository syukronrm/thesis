use std::collections::{BinaryHeap, HashMap};

use petgraph::graph::NodeIndex;

use crate::structure::edge::{Range, Object};
use crate::structure::voronoi::state::State;
use crate::structure::voronoi::Voronoi;
use crate::structure::*;

type ObjectId = i32;
type Queue = BinaryHeap<State>;

struct VisitedNodes(HashMap<NodeIndex, (f32, ObjectId)>);

impl VisitedNodes {
    fn new() -> VisitedNodes {
        VisitedNodes({ HashMap::new() })
    }

    #[allow(dead_code)]
    fn contains(self, node_id: NodeIndex) -> Option<(f32, ObjectId)> {
        if let Some((dist, object_id)) = self.0.get(&node_id) {
            Some((*dist, *object_id))
        } else {
            None
        }
    }

    fn insert(&mut self, k: NodeIndex, v: (f32, ObjectId)) {
        self.0.insert(k, v);
    }

    fn visited(&self, centroid_id: i32, node_id: NodeIndex) -> bool {
        if let Some((_, object_id)) = self.0.get(&node_id) {
            object_id == &centroid_id
        } else {
            false
        }
    }
}

fn enqueue_neighbors(queue: &mut Queue, g: &Graph, centroids: &[Object]) {
    for c in centroids {
        let edge_index = g.edge_index(c.edge_id);
        let (node_index_n, node_index_m) = g.graph.edge_endpoints(edge_index).unwrap();
        let edge = g.graph.edge_weight(edge_index).unwrap();
        let node_n = g.graph.node_weight(node_index_n).unwrap();
        if edge.ni == node_n.id {
            let dist = edge.len * c.dist;
            queue.push(State::new(node_index_n, c.id, dist));

            let dist = edge.len * (1.0 - c.dist);
            queue.push(State::new(node_index_m, c.id, dist));
        } else {
            let dist = edge.len * (1.0 - c.dist);
            queue.push(State::new(node_index_n, c.id, dist));

            let dist = edge.len * c.dist;
            queue.push(State::new(node_index_m, c.id, dist));
        }
    }
}

#[allow(dead_code, unused_variables, unused_mut)]
fn compute_voronoi(
    g: &Graph,
    src_centroid_id: ObjectId,
    centroids: Vec<Object>,
    max_distance: f32,
    dimensions: Vec<i8>,
) -> Vec<Voronoi> {
    let mut voronoi = Voronoi::new();
    let mut visited_nodes = VisitedNodes::new();
    let mut queue: Queue = BinaryHeap::new();
    let graph = &g.graph;
    let objects = g.objects.borrow();

    enqueue_neighbors(&mut queue, g, &centroids);

    while !queue.is_empty() {
        let state = queue.pop().unwrap();

        if state.dist < max_distance {
            continue;
        }

        let State {
            node_index: node_index_n,
            centroid_id,
            dist: dist_n,
        } = state;
        let centroid = objects.get(&src_centroid_id).unwrap();
        let neighbors = graph.neighbors(node_index_n);
        for node_index_m in neighbors {
            if visited_nodes.visited(centroid_id, node_index_m) {
                continue;
            }

            let edge_index_m = graph.find_edge(node_index_n, node_index_m).unwrap();
            let edge = graph.edge_weight(edge_index_m).unwrap();
            let dist_m = dist_n + edge.len;
            if let Some((dist, centroid_id)) = visited_nodes.0.get(&node_index_m) {
            } else {
                if dist_m < max_distance {
                    queue.push(State::new(node_index_m, centroid_id, dist_m));
                    visited_nodes.insert(node_index_m, (dist_m, centroid_id));
                }

                if centroid_id == src_centroid_id {
                    let node_n = graph.node_weight(node_index_n).unwrap();
                    let mut start;
                    let mut end;
                    if edge.ni == node_n.id {
                        start = 0.0;
                        if dist_n + edge.len > max_distance {
                            end = edge.len;
                        } else {
                            end = max_distance - dist_n;
                        }
                    } else {
                        start = edge.len;
                        if dist_n + edge.len > max_distance {
                            end = 0.0;
                        } else {
                            end = edge.len - (max_distance - dist_n);
                        }
                    }

                    let range = Range::new(start, end, centroid.clone());
                    voronoi.insert(edge_index_m, range);
                }
            }
        }
    }

    Vec::new()
}

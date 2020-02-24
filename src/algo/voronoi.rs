use std::collections::{BinaryHeap, HashMap};
use std::rc::Rc;

use crate::structure::edge::Object;
use crate::structure::voronoi::state::State;
use crate::structure::voronoi::Voronoi;
use crate::structure::*;

type ObjectId = i32;
type NodeId = i32;
type Queue = BinaryHeap<State>;

struct VisitedNodes(HashMap<NodeId, (f32, ObjectId, Rc<Edge>)>);

impl VisitedNodes {
    fn new() -> VisitedNodes {
        VisitedNodes({ HashMap::new() })
    }
}

fn enqueue_neighbors(queue: &mut Queue, graph: &Graph, centroids: &[Object]) {
    for c in centroids {
        let node_ids = graph.nodes_from_edge_id(c.edge_id);
        let edge = graph.edge(c.edge_id);
        for node_id in node_ids {
            if edge.ni == node_id {
                let dist = edge.len * c.dist;
                queue.push(State::new(node_id, c.id, dist));
            } else {
                let dist = edge.len * (1.0 - c.dist);
                queue.push(State::new(node_id, c.id, dist));
            }
        }
    }
}

#[allow(dead_code, unused_variables, unused_mut)]
fn compute_voronoi(
    graph: &Graph,
    src_centroid_id: ObjectId,
    centroids: Vec<Object>,
    max_distance: f32,
    dimensions: Vec<i8>,
) -> Vec<Voronoi> {
    let mut voronoi = Voronoi::new();
    let mut visited_nodes = VisitedNodes::new();
    let mut queue: Queue = BinaryHeap::new();

    enqueue_neighbors(&mut queue, graph, &centroids);

    Vec::new()
}

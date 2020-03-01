use std::collections::{BinaryHeap, HashMap};

use crate::structure::*;
use petgraph::graph::NodeIndex;

fn as_node_id(id: i32) -> i32 {
    id + 100000
}

#[allow(dead_code)]
fn as_object_id(id: i32) -> i32 {
    id - 100000
}

#[allow(dead_code)]
fn graph_with_centroids(g: &mut Graph, centroids: &Vec<Object>) {
    for object in centroids {
        let new_node = Node {
            id: as_node_id(object.id),
            lng: 0.0,
            lat: 0.0
        };
        let edge_index = g.edge_index(object.edge_id);
        let edge = g.graph.edge_weight(edge_index).unwrap();
        let node_i_index = g.node_index(edge.ni);
        let node_j_index = g.node_index(edge.nj);
        let edge_from_ni = Edge::new(100000, edge.len * object.dist, new_node.id, edge.ni);
        let edge_from_nj = Edge::new(100000, edge.len * object.dist, new_node.id, edge.nj);
        let index = g.graph.add_node(new_node);
        g.graph.add_edge(node_i_index, index, edge_from_ni);
        g.graph.add_edge(node_j_index, index, edge_from_nj);
        let mut map_node_index = g.map_node_index.borrow_mut();
        map_node_index.insert(as_node_id(object.id), index);
    }
}

#[allow(dead_code)]
fn voronoi(g: &Graph, centroid_ids: Vec<NodeIndex>, max_distance: f32) {
    let graph = &g.graph;

    // set all distance to the max
    let mut dist_map = {
        let mut map: HashMap<NodeIndex, f32> = HashMap::new();
        g.graph.node_indices().into_iter().for_each(|x| {
            map.insert(x, std::f32::MAX);
        });
        map
    };
    let mut heap: BinaryHeap<State> = BinaryHeap::new();

    // set centroid distances to 0 and insert centroid to heap queue
    for centroid_index in centroid_ids {
        let val = dist_map.get_mut(&centroid_index).unwrap();
        *val = 0.0;
        heap.push(State::new(centroid_index, centroid_index, 0.0));
    }

    while let Some(State { node_index, centroid_id, dist}) = heap.pop() {
        let existing_distance = dist_map.get(&node_index).unwrap();
        if dist > *existing_distance {
            continue;
        }

        let neighbors = graph.neighbors(node_index);
        for node_index_next in neighbors {
            let edge_index = graph.find_edge(node_index, node_index_next).unwrap();
            let edge = graph.edge_weight(edge_index).unwrap();
            let next = dist + edge.len;

            if next > max_distance {
                // add edge from start to max_distance to voronoi
                continue;
            }

            let next_existing = dist_map.get(&node_index_next).unwrap();
            if next < *next_existing {
                heap.push(State::new(node_index_next, centroid_id, next));
                let val = dist_map.get_mut(&node_index_next).unwrap();
                *val = next;
                // add all edge to voronoi
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_with_centroids() {
        let mut graph: PetgraphNodeEdge = petgraph::stable_graph::StableGraph::with_capacity(0, 0);
        let n1 = graph.add_node(Node { id: 1, lng: 0.0, lat: 0.0});
        let n2 = graph.add_node(Node { id: 2, lng: 3.0, lat: 4.0});
        graph.add_edge(n1, n2, Edge::new(1, 5.0, 1, 2));
        let objects = vec![
            Object { id: 1, attr: vec![1.0], edge_id: 1, dist: 0.5 }
        ];

        let mut g = Graph::new(graph);
        graph_with_centroids(&mut g, &objects);
        assert_eq!(g.graph.edge_indices().count(), 3);
        assert_eq!(g.graph.node_indices().count(), 3);
    }
}

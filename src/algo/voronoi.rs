use std::collections::{BinaryHeap, HashMap};

use crate::structure::*;
use petgraph::graph::NodeIndex;

fn as_node_id(id: i32) -> i32 {
    id + 100000
}

fn as_edge_ni_id(id: i32) -> i32 {
    id + 200000
}

fn as_edge_nj_id(id: i32) -> i32 {
    id + 300000
}

#[allow(dead_code)]
fn as_object_id(id: i32) -> i32 {
    id - 100000
}

#[allow(dead_code)]
fn graph_with_centroids(g: &mut Graph, centroids: &Vec<Object>) {
    let mut map_edge_objects: HashMap<i32, Vec<Object>> = HashMap::new();
    for c in centroids {
        if let Some(vec) = map_edge_objects.get_mut(&c.edge_id) {
            vec.push(c.clone());
        } else {
            map_edge_objects.insert(c.edge_id, vec![c.clone()]);
        }
    }

    let insert_centroid_as_node = |g: &mut Graph,
                                   left: &Node,
                                   right: &Node,
                                   object: &Object,
                                   lng: f32,
                                   lat: f32,
                                   is_last|
     -> NodeIndex {
        let node = Node {
            id: as_node_id(object.id),
            lng,
            lat,
        };

        let left_index = g.node_index(left.id);
        let edge_len_left = {
            let x = left.lng - node.lng;
            let y = left.lat - node.lat;
            (x.powi(2) + y.powi(2)).sqrt()
        };
        let edge_left = Edge::new(as_edge_ni_id(node.id), edge_len_left, left.id, node.id);
        let new_node = g.graph.add_node(node.clone());
        g.add_node_index(node.id, new_node);
        g.graph.add_edge(left_index, new_node, edge_left);

        if is_last {
            let right_index = g.node_index(right.id);
            let edge_len_right = {
                let x = right.lng - node.lng;
                let y = right.lat - node.lat;
                (x.powi(2) + y.powi(2)).sqrt()
            };
            let edge_right = Edge::new(as_edge_nj_id(node.id), edge_len_right, node.id, right.id);
            g.graph.add_edge(new_node, right_index, edge_right);
        }

        new_node
    };

    for (key, value) in &mut map_edge_objects {
        value.sort_by(|a, b| a.dist.partial_cmp(&b.dist).unwrap());

        let edge = g.edge(*key).clone();
        let node_ni = g.node(edge.ni).clone();
        let node_nj = g.node(edge.nj).clone();

        let mut left = g.node(edge.ni).clone();
        let right = g.node(edge.nj).clone();

        let mut iter = value.iter().peekable();
        while let Some(object) = iter.next() {
            let lng = (node_nj.lng - node_ni.lng) * object.dist + node_ni.lng;
            let lat = (node_nj.lat - node_ni.lat) * object.dist + node_ni.lat;
            let is_last = if let Some(_) = iter.peek() {
                false
            } else {
                true
            };
            let left_index = insert_centroid_as_node(g, &left, &right, object, lng, lat, is_last);
            left = g.graph.node_weight(left_index).unwrap().clone();
        }
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

    while let Some(State {
        node_index,
        centroid_id,
        dist,
    }) = heap.pop()
    {
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
        let n1 = graph.add_node(Node {
            id: 1,
            lng: 0.0,
            lat: 0.0,
        });
        let n2 = graph.add_node(Node {
            id: 2,
            lng: 3.0,
            lat: 4.0,
        });
        graph.add_edge(n1, n2, Edge::new(1, 5.0, 1, 2));
        let objects = vec![
            Object {
                id: 1,
                attr: vec![1.0],
                edge_id: 1,
                dist: 0.4,
            },
            Object {
                id: 2,
                attr: vec![1.0],
                edge_id: 1,
                dist: 0.8,
            },
        ];

        let mut g = Graph::new(graph);
        graph_with_centroids(&mut g, &objects);
        assert_eq!(g.graph.edge_indices().count(), 4);
        assert_eq!(g.graph.node_indices().count(), 4);

        for node_index in g.graph.node_indices() {
            println!("{:?}", g.graph.node_weight(node_index));
        }

        for edge_index in g.graph.edge_indices() {
            println!("{:?}", g.graph.edge_weight(edge_index));
        }
    }
}

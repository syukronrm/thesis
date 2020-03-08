use std::collections::{BinaryHeap, HashMap};
use std::rc::Rc;

use crate::structure::*;
use petgraph::graph::{EdgeIndex, NodeIndex};

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
pub fn graph_with_centroids(g: &mut Graph, centroids: &Vec<Rc<Object>>) -> Vec<NodeIndex> {
    let mut map_edge_objects: HashMap<i32, Vec<Rc<Object>>> = HashMap::new();
    let mut new_node_indices = Vec::new();
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
        let edge_id = edge_left.id;
        let edge_index = g.graph.add_edge(left_index, new_node, edge_left);
        g.add_edge_index(edge_id, edge_index);

        if is_last {
            let right_index = g.node_index(right.id);
            let edge_len_right = {
                let x = right.lng - node.lng;
                let y = right.lat - node.lat;
                (x.powi(2) + y.powi(2)).sqrt()
            };
            let edge_right = Edge::new(as_edge_nj_id(node.id), edge_len_right, node.id, right.id);
            let edge_id = edge_right.id;
            let edge_index = g.graph.add_edge(new_node, right_index, edge_right);
            g.add_edge_index(edge_id, edge_index);
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
            new_node_indices.push(left_index);
            left = g.graph.node_weight(left_index).unwrap().clone();
        }
    }

    new_node_indices
}

#[derive(Debug)]
pub struct VoronoiRange {
    pub start: f32,
    pub end: f32,
    pub centroid_id: NodeIndex,
}

impl VoronoiRange {
    pub fn print(&self, g: &Graph) {
        let centroid_id = g.graph.node_weight(self.centroid_id).unwrap().id;
        println!("Voronoi {{ start: {:?}, end: {:?}, centroid_id: {:?} }}", self.start, self.end, centroid_id);
    }
}

#[derive(Debug)]
pub struct Voronoi(HashMap<EdgeIndex, Vec<VoronoiRange>>);

impl Voronoi {
    pub fn new() -> Voronoi {
        Voronoi(HashMap::new())
    }

    pub fn insert(&mut self, edge: EdgeIndex, range: VoronoiRange) {
        if let Some(vr) = self.0.get_mut(&edge) {
            vr.push(range);
        } else {
            self.0.insert(edge, vec![range]);
        }
    }

    pub fn content(self) -> HashMap<EdgeIndex, Vec<VoronoiRange>> {
        self.0
    }
}

#[allow(dead_code)]
pub fn voronoi(g: &mut Graph, centroids: &Vec<Rc<Object>>, max_distance: f32) -> Voronoi {
    let new_node_indices = graph_with_centroids(g, centroids);
    let graph = &g.graph;
    let mut voronoi: Voronoi = Voronoi::new();

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
    for centroid_index in new_node_indices {
        let val = dist_map.get_mut(&centroid_index).unwrap();
        *val = 0.0;
        println!("heap.push {:?}", centroid_index);
        heap.push(State::new(centroid_index, centroid_index, 0.0));
    }

    while let Some(State {
        node_index,
        centroid_id,
        dist,
    }) = heap.pop()
    {
        println!("heap.pop {:?} centroid_id {:?}", node_index, centroid_id);
        let existing_distance = dist_map.get(&node_index).unwrap();
        if dist > *existing_distance {
            continue;
        }

        let node = g.graph.node_weight(node_index).unwrap();
        println!("  popped node.id {:?}", node.id);
        let neighbors = graph.neighbors(node_index);
        for node_index_next in neighbors {
            let edge_index = graph.find_edge(node_index, node_index_next).unwrap();
            let edge = graph.edge_weight(edge_index).unwrap();
            println!("    neigbor dist {:?} edge.id {:?} edge.len {:?}", dist, edge.id, edge.len);
            let next = dist + edge.len;

            let next_existing = dist_map.get(&node_index_next).unwrap();
            if next < *next_existing {
                println!("      heap.push {:?}", node_index_next);
                heap.push(State::new(node_index_next, centroid_id, next));
                let val = dist_map.get_mut(&node_index_next).unwrap();
                *val = next;

                if dist < max_distance {
                    if next > max_distance {
                        println!("        1 node.id {:?} edge.id {:?} edge.len {:?} next {:?}", node.id, edge.id, edge.len, next);
                        let start = if edge.ni == node.id { 0.0 } else { edge.len };
                        let end = if edge.ni == node.id {
                            max_distance - dist
                        } else {
                            edge.len - (max_distance - dist)
                        };
        
                        let range = VoronoiRange { start, end, centroid_id };
                        println!("        voronoi edge_index {:?} range insert {:?}", edge_index, range);
                        voronoi.insert(edge_index, range);
                    } else {
                        println!("        2 node.id {:?} edge.id {:?} edge.len {:?} next {:?}", node.id, edge.id, edge.len, next);
                        let start = if edge.ni == node.id { 0.0 } else { edge.len };
                        let end = if edge.ni == node.id { next } else { 0.0 };
        
                        let range = VoronoiRange { start, end, centroid_id };
                        println!("        voronoi edge_index {:?} range insert {:?}", edge_index, range);
                        voronoi.insert(edge_index, range);
                    }
                }
            }
        }
    }

    voronoi
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
            Rc::new(Object {
                id: 1,
                attr: vec![1.0],
                edge_id: 1,
                dist: 0.4,
            }),
            Rc::new(Object {
                id: 2,
                attr: vec![1.0],
                edge_id: 1,
                dist: 0.8,
            }),
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

    #[test]
    fn test_voronoi() {
        use std::path::Path;
        use crate::create_initial_graph;

        let project_path = Path::new(env!("CARGO_MANIFEST_DIR"));
        let dataset_dir = project_path.join("dataset/test01");
        let node_csv = "node.txt";
        let edge_csv = "edge.txt";
        let mut graph = create_initial_graph(dataset_dir, node_csv, edge_csv);

        let objects = vec![
            Rc::new(Object {
                id: 1,
                attr: vec![1.0],
                edge_id: 4,
                dist: 0.3535533,
            }),
            Rc::new(Object {
                id: 2,
                attr: vec![1.0],
                edge_id: 1,
                dist: 0.8,
            }),
        ];

        let voronoi = voronoi(&mut graph, &objects, 100.0);
        for v in voronoi.0.into_iter() {
            println!("{:?}", v);
        }
        println!("{:?}", graph.map_edge_index);
        println!("{:?}", graph.map_node_index);

        // for edge_index in graph.graph.edge_indices() {
        //     println!("{:?}", graph.graph.edge_weight(edge_index));
        // }
    }
}

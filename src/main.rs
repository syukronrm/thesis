mod dataset;
mod structure;

use petgraph::{Graph as PetGraph, Undirected};
use std::collections::HashMap;
use std::rc::Rc;

use dataset::Action::*;
use dataset::Edge as DataEdge;
use dataset::*;

use structure::{Edge, Graph, Node};
pub type GraphNodeEdge = PetGraph<Node, Edge, Undirected>;

#[allow(dead_code)]
fn prepare_graph(edges: Vec<DataEdge>) -> GraphNodeEdge {
    let mut graph: GraphNodeEdge = PetGraph::new_undirected();
    let mut added_node_ids = HashMap::new();

    let mut get_node_index = move |node: Rc<dataset::Node>, graph: &mut GraphNodeEdge| {
        match added_node_ids.get(&node.id) {
            Some(node_index) => *node_index,
            None => {
                let node_index = graph.add_node(Node {
                    id: node.id,
                    lng: node.lng,
                    lat: node.lat,
                });
                added_node_ids.insert(node.id, node_index);
                node_index
            }
        }
    };

    for edge in edges {
        let graph_ni = get_node_index(edge.ni, &mut graph);
        let graph_nj = get_node_index(edge.nj, &mut graph);
        graph.add_edge(graph_ni, graph_nj, Edge::new(edge.id, edge.len));
    }

    graph
}

fn get_all_edges() -> Vec<DataEdge> {
    use std::path::Path;

    let project_path = Path::new(env!("CARGO_MANIFEST_DIR"));
    let node_path = project_path.join("dataset/california/normalized/cal.cnode.txt");
    let edge_path = project_path.join("dataset/california/normalized/cal.cedge.txt");
    let nodes = dataset::read_node_csv(&node_path);
    dataset::read_edge_csv(&edge_path, &nodes)
}

fn main() {
    let _objects = vec![
        NewObject::new(1, vec![1.0, 8.0, 6.0, 7.0], 10.0, 4, Insertion),
        NewObject::new(2, vec![5.0, 7.0, 1.0, 3.0], 40.0, 5, Insertion),
        NewObject::new(3, vec![5.0, 1.0, 4.0, 5.0], 60.0, 3, Insertion),
        NewObject::new(4, vec![3.0, 4.0, 3.0, 9.0], 20.0, 3, Insertion),
        NewObject::new(5, vec![4.0, 4.0, 4.0, 4.0], 80.0, 3, Insertion),
    ];

    let edges = get_all_edges();
    let graph = prepare_graph(edges);
    let _structure = Graph::new(graph);

    println!("Hello, world!");
}

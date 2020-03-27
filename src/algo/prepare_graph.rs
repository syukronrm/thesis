use petgraph::stable_graph::StableGraph;
use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;

use crate::dataset::{Edge, Node};
use crate::structure::{EdgeGraph, Graph, NodeGraph, PetgraphNodeEdge};

use crate::dataset::load_edges;

#[allow(dead_code)]
pub fn create_initial_graph(dataset_dir: PathBuf, node_csv: &str, edge_csv: &str) -> Graph {
    let edges = load_edges(dataset_dir, node_csv, edge_csv);
    prepare_graph(edges)
}

#[allow(dead_code)]
fn prepare_graph(edges: Vec<Edge>) -> Graph {
    let mut graph: PetgraphNodeEdge = StableGraph::with_capacity(0, 0);
    let mut added_node_ids = HashMap::new();

    let mut get_node_index =
        move |node: &Rc<Node>, graph: &mut PetgraphNodeEdge| match added_node_ids.get(&node.id)
        {
            Some(node_index) => *node_index,
            None => {
                let node_index = graph.add_node(NodeGraph {
                    id: node.id,
                    lng: node.lng,
                    lat: node.lat,
                });
                added_node_ids.insert(node.id, node_index);
                node_index
            }
        };

    for edge in edges {
        let graph_ni = get_node_index(&edge.ni, &mut graph);
        let graph_nj = get_node_index(&edge.nj, &mut graph);
        graph.add_edge(
            graph_ni,
            graph_nj,
            EdgeGraph::new(edge.id, edge.len, edge.ni.id, edge.nj.id),
        );
    }

    Graph::new(graph)
}

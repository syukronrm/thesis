use petgraph::Graph as PetGraph;
use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;

use crate::dataset::{Edge as DataEdge, Node as DataNode};
use crate::structure::{Graph, Edge, PetgraphNodeEdge, Node};

use crate::dataset::load_edges;

#[allow(dead_code)]
pub fn create_initial_graph() -> Graph {
    let project_path = Path::new(env!("CARGO_MANIFEST_DIR"));
    let dataset_dir = project_path.join("dataset/california/normalized");
    let edges = load_edges(dataset_dir, "cal.cnode.txt", "cal.cedge.txt");
    prepare_graph(edges)
}

#[allow(dead_code)]
fn prepare_graph(edges: Vec<DataEdge>) -> Graph {
    let mut graph = PetGraph::new_undirected();
    let mut added_node_ids = HashMap::new();

    let mut get_node_index =
        move |node: Rc<DataNode>, graph: &mut PetgraphNodeEdge| match added_node_ids.get(&node.id) {
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
        };

    for edge in edges {
        let graph_ni = get_node_index(edge.ni, &mut graph);
        let graph_nj = get_node_index(edge.nj, &mut graph);
        graph.add_edge(graph_ni, graph_nj, Edge::new(edge.id, edge.len));
    }

    Graph::new(graph)
}
mod dataset;
mod graph;

use std::cell::RefCell;

use dataset::Action::*;
use dataset::*;

use graph::{Edge, Node};
use petgraph::Graph as PetGraph;
use petgraph::Undirected;

type GraphRef = RefCell<PetGraph<Edge, Node, Undirected>>;

#[allow(dead_code)]
fn prepare_graph(_nodes: Vec<dataset::Node>, _edges: Vec<dataset::Edge>) -> GraphRef {
    let _graph: GraphRef = RefCell::new(PetGraph::new_undirected());

    // TODO insert nodes
    // TODO insert edges

    _graph
}

fn main() {
    let _objects = vec![
        NewObject::new(1, vec![1.0, 8.0, 6.0, 7.0], 10.0, 4, Insertion),
        NewObject::new(2, vec![5.0, 7.0, 1.0, 3.0], 40.0, 5, Insertion),
        NewObject::new(3, vec![5.0, 1.0, 4.0, 5.0], 60.0, 3, Insertion),
        NewObject::new(4, vec![3.0, 4.0, 3.0, 9.0], 20.0, 3, Insertion),
        NewObject::new(5, vec![4.0, 4.0, 4.0, 4.0], 80.0, 3, Insertion),
    ];

    println!("Hello, world!");
}

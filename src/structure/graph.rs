use crate::structure::{Edge, Node, PetgraphNodeEdge};

use std::cell::RefCell;
use std::collections::HashMap;

use petgraph::graph::NodeIndex;
use petgraph::{Graph as PetGraph, Undirected};

type NodeId = i32;

pub struct Graph {
    graph: PetGraph<Node, Edge, Undirected>,
    map_node_index: RefCell<HashMap<NodeId, NodeIndex>>,
}

impl Graph {
    pub fn new(graph: PetgraphNodeEdge) -> Graph {
        let s = Graph {
            graph,
            map_node_index: RefCell::new(HashMap::new()),
        };
        s.recompute_node_index();
        s
    }

    fn recompute_node_index(&self) {
        self.map_node_index.replace(
            self.graph
                .node_indices()
                .map(|index| (self.graph[index].id, index))
                .collect(),
        );
    }
}
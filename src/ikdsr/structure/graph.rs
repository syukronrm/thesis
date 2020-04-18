use crate::prelude::*;
use petgraph::stable_graph::{Neighbors, NodeIndices, StableGraph};
use petgraph::Undirected;
use std::sync::Arc;

pub struct Graph {
    pub config: Arc<AppConfig>,
    inner: StableGraph<Node, Edge, Undirected>,
}

impl Graph {
    pub fn node_indices(&self) -> NodeIndices<Node> {
        self.inner.node_indices()
    }

    pub fn find_edge(&self, a: NodeIndex, b: NodeIndex) -> EdgeIndex {
        self.inner.find_edge(a, b).unwrap()
    }

    pub fn neighbors(&self, node: NodeIndex) -> Neighbors<Edge> {
        self.inner.neighbors(node)
    }

    pub fn edge_len(&self, edge: EdgeIndex) -> f32 {
        self.inner.edge_weight(edge).unwrap().len
    }
}

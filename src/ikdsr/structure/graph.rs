use crate::prelude::*;
use petgraph::stable_graph::{EdgeIndices, Neighbors, NodeIndices, StableGraph};
use petgraph::Undirected;
use std::collections::HashMap;
use std::sync::Arc;

pub struct Graph {
    pub config: Arc<AppConfig>,
    inner: StableGraph<Node, Edge, Undirected>,
}

impl Graph {
    pub fn new(config: Arc<AppConfig>) -> Self {
        let mut graph = StableGraph::with_capacity(0, 0);
        let mut itself = Graph {
            config,
            inner: graph,
        };
        itself.initial_network();
        itself
    }

    fn initial_network(&mut self) {
        let conf: AppConfig = Default::default();
        let conf = Arc::new(conf);
        let reader = Reader::new(conf);
        let arc_nodes = reader.read_node_csv();

        let nodes: Vec<Node> = arc_nodes
            .clone()
            .into_iter()
            .map(|n| Node { id: n.id })
            .collect();

        let mut map_node_index: HashMap<NodeId, NodeIndex> = HashMap::new();
        for node in nodes {
            let id = node.id;
            let node_index = self.inner.add_node(node);
            map_node_index.insert(id, node_index);
        }

        let edges = reader.read_edge_csv(&arc_nodes);

        for edge in edges {
            let node_i = map_node_index.get(&edge.ni.id).unwrap();
            let node_j = map_node_index.get(&edge.nj.id).unwrap();
            self.inner
                .add_edge(*node_i, *node_j, Edge::new(edge.id, edge.len));
        }
    }

    pub fn node_indices(&self) -> NodeIndices<Node> {
        self.inner.node_indices()
    }

    #[cfg(test)]
    pub fn edge_indices(&self) -> EdgeIndices<Edge> {
       self.inner.edge_indices()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn graph_new() {
        let conf = Arc::new(AppConfig::default());
        let graph = Graph::new(conf);
        let node_indices = graph.node_indices().into_iter().count();
        assert_eq!(node_indices, 6);

        let edge_indices = graph.edge_indices().into_iter().count();
        assert_eq!(edge_indices, 5);
    }
}
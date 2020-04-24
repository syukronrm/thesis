use crate::prelude::*;
use petgraph::stable_graph::{EdgeIndices, Neighbors, NodeIndices, StableGraph};
use petgraph::Undirected;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone)]
pub struct Graph {
    pub config: Arc<AppConfig>,
    map_node_index: HashMap<NodeId, NodeIndex>,
    objects: HashMap<ObjectId, Arc<DataObject>>,
    inner: StableGraph<Node, Edge, Undirected>,
}

impl Graph {
    pub fn new(config: Arc<AppConfig>) -> Self {
        let graph = StableGraph::with_capacity(0, 0);
        let mut itself = Graph {
            config,
            map_node_index: HashMap::new(),
            objects: HashMap::new(),
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

        let mut map_edge_index = HashMap::new();
        for edge in edges {
            let node_i = map_node_index.get(&edge.ni.id).unwrap();
            let node_j = map_node_index.get(&edge.nj.id).unwrap();
            let edge_index = self
                .inner
                .add_edge(*node_i, *node_j, Edge::new(edge.id, edge.len));
            map_edge_index.insert(edge.id, edge_index);
        }

        let objects = reader.read_object_csv();
        for object in objects {
            let edge_index = map_edge_index.get(&object.edge_id).unwrap();
            let edge = self.inner.edge_weight_mut(*edge_index).unwrap();
            edge.add_object(object.clone());
            self.objects.insert(object.id, object);
        }

        self.map_node_index = map_node_index;
    }

    // TODO: insert object as node, returning NodeIndex
    #[allow(dead_code)]
    fn insert_object_as_node(&mut self, object: Arc<DataObject>) {}

    pub fn node_indices(&self) -> NodeIndices<Node> {
        self.inner.node_indices()
    }

    pub fn node_id(&self, node_index: NodeIndex) -> NodeId {
        self.inner.node_weight(node_index).unwrap().id
    }

    pub fn edge_id(&self, edge_index: EdgeIndex) -> EdgeId {
        self.inner.edge_weight(edge_index).unwrap().id
    }

    pub fn node_index(&self, node_id: NodeId) -> NodeIndex {
        *self.map_node_index.get(&node_id).unwrap()
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

    pub fn objects(&self, edge: EdgeIndex) -> Vec<Arc<DataObject>> {
        self.inner.edge_weight(edge).unwrap().objects.clone()
    }

    #[cfg(test)]
    pub fn edge_indices(&self) -> EdgeIndices<Edge> {
        self.inner.edge_indices()
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

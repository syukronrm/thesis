use crate::prelude::*;
use petgraph::graphmap::{GraphMap, Neighbors, Nodes};
use petgraph::Undirected;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone)]
pub struct Graph {
    pub config: Arc<AppConfig>,
    objects: HashMap<ObjectId, Arc<DataObject>>,
    inner: GraphMap<NodeId, Edge, Undirected>,
}

impl Graph {
    pub fn new(config: Arc<AppConfig>) -> Self {
        let graph = GraphMap::new();
        let mut itself = Graph {
            config,
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

        let edges = reader.read_edge_csv(&arc_nodes);

        let mut map_edges = HashMap::new();
        for edge in edges {
            self.inner
                .add_edge(edge.ni.id, edge.nj.id, Edge::new(edge.id, edge.len));
            map_edges.insert(edge.id, edge);
        }

        let objects = reader.read_object_csv();
        for object in objects {
            let edge_data = map_edges.get(&object.edge_id).unwrap();
            let edge = self
                .inner
                .edge_weight_mut(edge_data.ni.id, edge_data.nj.id)
                .unwrap();
            edge.add_object(object.clone());
            self.objects.insert(object.id, object);
        }
    }

    // TODO: insert object as node, returning NodeIndex
    #[allow(dead_code, unused_variables)]
    fn insert_object_as_node(&mut self, object: Arc<DataObject>) {}

    pub fn neighbors(&self, n: NodeId) -> Neighbors<NodeId> {
        self.inner.neighbors(n)
    }

    pub fn nodes(&self) -> Nodes<NodeId> {
        self.inner.nodes()
    }

    pub fn edge_len(&self, a: NodeId, b: NodeId) -> f32 {
        let edge = self.inner.edge_weight(a, b).unwrap();
        edge.len
    }

    pub fn objects(&self, a: NodeId, b: NodeId) -> Vec<Arc<DataObject>> {
        let edge_weight = self.inner.edge_weight(a, b).unwrap();
        edge_weight.objects.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn graph_new() {
        let conf = Arc::new(AppConfig::default());
        let graph = Graph::new(conf);
        let nodes = graph.inner.nodes().into_iter().count();
        assert_eq!(nodes, 6);

        let edges = graph.inner.all_edges().into_iter().count();
        assert_eq!(edges, 5);
    }
}

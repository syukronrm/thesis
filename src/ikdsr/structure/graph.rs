use crate::prelude::*;
use petgraph::graphmap::{GraphMap, Neighbors, Nodes};
use petgraph::Undirected;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone)]
pub struct Graph {
    pub config: Arc<AppConfig>,
    objects: HashMap<ObjectId, Arc<DataObject>>,
    map_nodes: HashMap<NodeId, Arc<DataNode>>,
    map_edges: HashMap<EdgeId, Arc<DataEdge>>,
    inner: GraphMap<NodeId, Edge, Undirected>,
}

impl Graph {
    pub fn new(config: Arc<AppConfig>) -> Self {
        let graph = GraphMap::new();
        let mut itself = Graph {
            config,
            objects: HashMap::new(),
            map_nodes: HashMap::new(),
            map_edges: HashMap::new(),
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
        self.map_nodes = arc_nodes.iter().map(|a| (a.id, a.clone())).collect();

        let edges = reader.read_edge_csv(&arc_nodes);
        for edge in edges {
            self.inner.add_edge(
                edge.ni,
                edge.nj,
                Edge::new(edge.id, edge.len, edge.ni, edge.nj),
            );
            self.map_edges.insert(edge.id, edge);
        }

        let objects = reader.read_object_csv();
        for object in objects {
            let edge_data = self.map_edges.get(&object.edge_id).unwrap();
            let edge = self
                .inner
                .edge_weight_mut(edge_data.ni, edge_data.nj)
                .unwrap();
            edge.add_object(object.clone());
            self.objects.insert(object.id, object);
        }
    }

    pub fn convert_object_as_node(&mut self, object: Arc<DataObject>) -> NodeId {
        let new_node_ids = self.convert_objects_as_node_in_edge(object.edge_id, vec![object]);
        *new_node_ids.first().unwrap()
    }

    pub fn convert_objects_to_node(&mut self, mut objects: Vec<Arc<DataObject>>) -> Vec<NodeId> {
        let mut map_edge_id: HashMap<EdgeId, Vec<Arc<DataObject>>> = HashMap::new();

        for o in objects {
            if let Some(vec_arc) = map_edge_id.get_mut(&o.edge_id) {
                vec_arc.push(o);
            } else {
                map_edge_id.insert(o.edge_id, vec![o]);
            }
        }

        let mut node_ids = Vec::new();
        for (edge_id, vec_arc) in map_edge_id {
            let res = self.convert_objects_as_node_in_edge(edge_id, vec_arc);
            node_ids.append(&mut res.clone());
        }
        node_ids
    }

    #[allow(dead_code)]
    fn convert_objects_as_node_in_edge(
        &mut self,
        edge_id: EdgeId,
        mut objects: Vec<Arc<DataObject>>,
    ) -> Vec<NodeId> {
        let edge = self.map_edges.get(&edge_id).unwrap();
        let edge = self.inner.edge_weight(edge.ni, edge.nj).unwrap().clone();
        objects.sort_by(|a, b| a.dist.partial_cmp(&b.dist).unwrap());

        let last_object = objects.last();

        let mut prev_node_id = edge.ni;
        let last_node_id = edge.nj;
        let mut new_node_ids = Vec::new();
        let mut prev_dist = 0.0;

        for o in &objects {
            let new_node_id = Self::as_node_id(o);
            self.inner.add_node(new_node_id);
            new_node_ids.push(new_node_id);

            // keep new node id mapped
            let node = self.create_node_from_object(new_node_id, o.dist, &edge);
            self.map_nodes.insert(node.id, node);

            // insert new edge before object
            let prev_edge_id = Self::object_as_edge_id(o.id);
            let objects = edge.objects_in_between(prev_dist, o.dist);
            self.add_edge(prev_edge_id, prev_node_id, new_node_id, objects);

            // insert new edge after object
            if last_object.unwrap().id == o.id {
                let next_edge_id = Self::object_as_last_edge_id(o.id);
                let objects = edge.objects_in_between(o.dist, 1.0);
                self.add_edge(next_edge_id, new_node_id, last_node_id, objects);
            }

            prev_node_id = new_node_id;
            prev_dist = o.dist;
        }

        new_node_ids
    }

    fn create_node_from_object(
        &self,
        node_id: NodeId,
        object_dist: f32,
        edge: &Edge,
    ) -> Arc<DataNode> {
        let ni = self.map_nodes.get(&edge.ni).unwrap();
        let nj = self.map_nodes.get(&edge.nj).unwrap();
        let lng = (nj.lng - ni.lng) * object_dist + ni.lng;
        let lat = (nj.lat - ni.lat) * object_dist + ni.lat;
        Arc::new(DataNode {
            id: node_id,
            lng,
            lat,
        })
    }

    fn add_edge(
        &mut self,
        edge_id: EdgeId,
        prev_node_id: NodeId,
        node_id: NodeId,
        objects: Vec<Arc<DataObject>>,
    ) {
        let edge_len = self.node_distance(prev_node_id, node_id);
        let mut new_edge = Edge::new(edge_id, edge_len, prev_node_id, node_id);
        new_edge.add_objects(objects);
        self.inner.add_edge(prev_node_id, node_id, new_edge);
    }

    fn node_distance(&self, a: NodeId, b: NodeId) -> f32 {
        let a = self.map_nodes.get(&a).unwrap();
        let b = self.map_nodes.get(&b).unwrap();
        let lng = a.lng - b.lng;
        let lat = a.lat - b.lat;
        (lng * lng + lat * lat).sqrt()
    }

    pub fn neighbors(&self, n: NodeId) -> Neighbors<NodeId> {
        self.inner.neighbors(n)
    }

    /// Remove node and its adjacent edges. Remove their indices too.
    pub fn remove_node(&mut self, n: NodeId) {
        let edges = self.inner.edges(n);
        for (_, _, e) in edges {
            self.map_edges.remove(&e.id);
        }
        self.map_nodes.remove(&n);
        self.inner.remove_node(n);
    }

    pub fn object(&self, object_id: ObjectId) -> Arc<DataObject> {
        self.objects.get(&object_id).unwrap().clone()
    }

    pub fn nodes(&self) -> Nodes<NodeId> {
        self.inner.nodes()
    }

    pub fn edge_len(&self, a: NodeId, b: NodeId) -> f32 {
        let edge = self.inner.edge_weight(a, b).unwrap();
        edge.len
    }

    pub fn edge_id(&self, a: NodeId, b: NodeId) -> Option<EdgeId> {
       if let Some(edge) = self.inner.edge_weight(a, b) {
           Some(edge.id)
       } else {
           None
       }
    }

    pub fn objects(&self, a: NodeId, b: NodeId) -> Vec<Arc<DataObject>> {
        let edge_weight = self.inner.edge_weight(a, b).unwrap();
        edge_weight.objects.clone()
    }

    fn as_node_id(o: &Arc<DataObject>) -> NodeId {
        o.id + 100000
    }

    fn as_object_id(node_id: NodeId) -> ObjectId {
        node_id - 100000
    }

    fn object_as_edge_id(object_id: ObjectId) -> EdgeId {
        object_id + 200000
    }

    fn object_as_last_edge_id(object_id: ObjectId) -> EdgeId {
        object_id + 300000
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

    #[test]
    fn convert_objects_as_node() {
        let conf = Arc::new(AppConfig::default());
        let mut graph = Graph::new(conf);

        fn new_object(id: ObjectId, edge_id: EdgeId, dist: f32) -> Arc<DataObject> {
            let o = DataObject {
                id,
                edge_id,
                dist,
                attr: Vec::new(),
                action: Action::Insertion,
            };
            Arc::new(o)
        }

        let objects = vec![
            new_object(100, 3, 0.3),
            new_object(101, 3, 0.4),
            new_object(102, 3, 0.8),
        ];

        let new_node_ids = graph.convert_objects_as_node_in_edge(3, objects);

        assert_eq!(new_node_ids.len(), 3);

        let edges = graph.inner.all_edges();
        let (mut e1, mut e2, mut e3) = (false, false, false);
        for (ni, nj, edge) in edges {
            if edge.id == Graph::object_as_edge_id(100) {
                e1 = true;
            } else if edge.id == Graph::object_as_edge_id(101) {
                e2 = true;
            } else if edge.id == Graph::object_as_last_edge_id(102) {
                e3 = true;
            }

            println!(
                "EdgeID {} \t NodeI {} \t NodeJ {} \t Length {}",
                edge.id, ni, nj, edge.len
            );

            println!("Objects: {:#?}", edge.objects);
        }

        assert!(e1 == true && e2 == true && e3 == true)
    }
}

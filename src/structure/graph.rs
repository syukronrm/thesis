use crate::structure::PetgraphNodeEdge;

use std::collections::HashMap;
use std::rc::Rc;

use crate::structure::edge::Object;
use crate::structure::*;
use petgraph::graph::{EdgeIndex, NodeIndex};

type NodeId = i32;
type EdgeId = i32;
type ObjectId = i32;

pub struct Graph {
    pub graph: PetgraphNodeEdge,
    pub objects: HashMap<ObjectId, Rc<Object>>,
    pub map_node_index: HashMap<NodeId, NodeIndex>,
    pub map_edge_index: HashMap<EdgeId, EdgeIndex>,
    pub queries: Multiqueries
}

impl Graph {
    pub fn new(graph: PetgraphNodeEdge) -> Graph {
        let mut s = Graph {
            graph,
            map_node_index: HashMap::new(),
            map_edge_index: HashMap::new(),
            objects: HashMap::new(),
            queries: Multiqueries::new(),
        };
        s.recompute_node_index();
        s.recompute_edge_index();
        s
    }

    pub fn insert(&self, object: Rc<Object>) {
        let pairs = self.queries.pairs();
        for pair in pairs {

        }
    }

    pub fn assign_queries(&mut self, multiqueries: Multiqueries) {
        self.queries = multiqueries;
    }

    pub fn dominator_and_dominated_objects(&self, pair: Pair, object: Rc<Object>) {
        let mut dist_map = {
            let mut map: HashMap<NodeIndex, (f32, Option<NodeIndex>)> = HashMap::new();
            self.graph.node_indices().into_iter().for_each(|x| {
                map.insert(x, (std::f32::MAX, None));
            });
            map
        };

        let edge_id = object.edge_id;
    }

    fn recompute_node_index(&mut self) {
        self.map_node_index = self.graph
                                .node_indices()
                                .map(|index| (self.graph[index].id, index))
                                .collect();
    }

    fn recompute_edge_index(&mut self) {
        self.map_edge_index = self.graph
                                .edge_indices()
                                .map(|index| (self.graph[index].id, index))
                                .collect();
    }

    pub fn edge_index(&self, edge_id: i32) -> EdgeIndex {
        *self.map_edge_index.get(&edge_id).unwrap()
    }

    pub fn edge(&self, edge_id: i32) -> &Edge {
        let edge_index = self.edge_index(edge_id);
        self.graph.edge_weight(edge_index).unwrap()
    }

    pub fn node_index(&self, node_id: i32) -> NodeIndex {
        *self.map_node_index.get(&node_id).unwrap()
    }

    pub fn node(&self, node_id: i32) -> &Node {
        let node_index = self.map_node_index.get(&node_id).unwrap();
        self.graph.node_weight(*node_index).unwrap()
    }

    #[allow(dead_code)]
    pub fn nodes_from_edge_id(&self, edge_id: EdgeId) -> Vec<i32> {
        let edge_index = self.map_edge_index.get(&edge_id).unwrap();
        let edge = self.graph.edge_weight(*edge_index).unwrap();
        vec![edge.ni, edge.nj]
    }

    #[allow(dead_code)]
    pub fn nodes_from_edge_ids(&self, edge_ids: &[EdgeId]) -> Vec<i32> {
        edge_ids
            .iter()
            .flat_map(|e| self.nodes_from_edge_id(*e))
            .collect()
    }

    #[allow(dead_code)]
    pub fn get_objects(&self, object_ids: Vec<ObjectId>) -> Vec<Rc<Object>> {
        object_ids
            .into_iter()
            .map(move |oid| {
                self.objects.get(&oid).unwrap().clone()
            })
            .rev()
            .collect()
    }

    #[allow(dead_code)]
    pub fn insert_object(&mut self, object: Rc<Object>) {
        self.objects.insert(object.id, object.clone());
    }

    #[allow(dead_code)]
    pub fn insert_objects(&mut self, objects: Vec<Rc<Object>>) {
        for o in objects {
            self.insert_object(o);
        }
    }

    pub fn add_node_index(&mut self, node_id: i32, node_index: NodeIndex) {
        if let Some(val) = self.map_node_index.get_mut(&node_id) {
            *val = node_index;
        } else {
            self.map_node_index.insert(node_id, node_index);
        }
    }

    pub fn add_edge_index(&mut self, edge_id: i32, edge_index: EdgeIndex) {
        if let Some(val) = self.map_edge_index.get_mut(&edge_id) {
            *val = edge_index;
        } else {
            self.map_edge_index.insert(edge_id, edge_index);
        }
    }
}

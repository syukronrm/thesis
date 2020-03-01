use crate::structure::PetgraphNodeEdge;

use std::cell::RefCell;
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
    pub objects: RefCell<HashMap<ObjectId, Rc<Object>>>,
    pub map_node_index: RefCell<HashMap<NodeId, NodeIndex>>,
    pub map_edge_index: RefCell<HashMap<EdgeId, EdgeIndex>>,
}

impl Graph {
    pub fn new(graph: PetgraphNodeEdge) -> Graph {
        let s = Graph {
            graph,
            map_node_index: RefCell::new(HashMap::new()),
            map_edge_index: RefCell::new(HashMap::new()),
            objects: RefCell::new(HashMap::new()),
        };
        s.recompute_node_index();
        s.recompute_edge_index();
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

    fn recompute_edge_index(&self) {
        self.map_edge_index.replace(
            self.graph
                .edge_indices()
                .map(|index| (self.graph[index].id, index))
                .collect(),
        );
    }

    pub fn edge_index(&self, edge_id: i32) -> EdgeIndex {
        let map_edge_index = self.map_edge_index.borrow();
        *map_edge_index.get(&edge_id).unwrap()
    }

    pub fn edge(&self, edge_id: i32) -> &Edge {
        let edge_index = self.edge_index(edge_id);
        self.graph.edge_weight(edge_index).unwrap()
    }

    pub fn node_index(&self, node_id: i32) -> NodeIndex {
        let map_node_index = self.map_node_index.borrow();
        *map_node_index.get(&node_id).unwrap()
    }

    pub fn node(&self, node_id: i32) -> &Node {
        let map_node_index = self.map_node_index.borrow();
        let node_index = map_node_index.get(&node_id).unwrap();
        self.graph.node_weight(*node_index).unwrap()
    }

    pub fn nodes_from_edge_id(&self, edge_id: EdgeId) -> Vec<i32> {
        let map_edge_index = self.map_edge_index.borrow();
        let edge_index = map_edge_index.get(&edge_id).unwrap();
        let edge = self.graph.edge_weight(*edge_index).unwrap();
        vec![edge.ni, edge.nj]
    }

    pub fn nodes_from_edge_ids(&self, edge_ids: &[EdgeId]) -> Vec<i32> {
        edge_ids
            .iter()
            .flat_map(|e| self.nodes_from_edge_id(*e))
            .collect()
    }

    pub fn get_objects(&self, object_ids: Vec<ObjectId>) -> Vec<Rc<Object>> {
        object_ids
            .into_iter()
            .map(move |oid| {
                let objects = self.objects.borrow();
                objects.get(&oid).unwrap().clone()
            })
            .rev()
            .collect()
    }

    pub fn insert_object(&self, object: Rc<Object>) {
        let mut objects = self.objects.borrow_mut();
        objects.insert(object.id, object.clone());
    }

    pub fn insert_objects(&self, objects: Vec<Rc<Object>>) {
        for o in objects {
            self.insert_object(o);
        }
    }
}

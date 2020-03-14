use crate::structure::PetgraphNodeEdge;

use std::collections::HashMap;
use std::rc::Rc;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

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
    pub queries: Multiqueries,
    max_dist: f32,
}

impl Graph {
    pub fn new(graph: PetgraphNodeEdge) -> Graph {
        let mut s = Graph {
            graph,
            map_node_index: HashMap::new(),
            map_edge_index: HashMap::new(),
            objects: HashMap::new(),
            queries: Multiqueries::new(),
            max_dist: 100.0,
        };
        s.recompute_node_index();
        s.recompute_edge_index();
        s
    }

    #[allow(dead_code,unused_variables)]
    pub fn insert(&mut self, object: Rc<Object>) {
        let pairs = self.queries.pairs();
        for pair in pairs {
            let (dominator, dominated) = self.dominator_and_dominated_objects(pair, object.clone());
        }
        self.insert_object_to_graph(object.clone());
    }

    pub fn assign_queries(&mut self, multiqueries: Multiqueries) {
        self.queries = multiqueries;
    }

    // get dominated and dominator objects measured in pair of dimension
    fn dominator_and_dominated_objects(&self, pair: Pair, object: Rc<Object>) -> (Vec<Rc<Object>>, Vec<Rc<Object>>) {
        let mut dist_map = {
            let mut map: HashMap<NodeIndex, f32> = HashMap::new();
            self.graph.node_indices().into_iter().for_each(|x| {
                map.insert(x, std::f32::MAX);
            });
            map
        };

        let mut dominator = Vec::new();
        let mut dominated = Vec::new();

        let mut queue = BinaryHeap::new();

        let edge_id = object.edge_id;
        let edge_index = self.map_edge_index.get(&edge_id).unwrap();
        let edge = self.graph.edge_weight(*edge_index).unwrap();

        for o in edge.objects.borrow().iter() {
            match object.dominate_pair(o.clone(), &pair) {
                Domination::Dominator => dominated.push(o.clone()),
                Domination::Dominated => dominator.push(o.clone()),
                _ => (),
            }
        }

        match self.graph.edge_endpoints(*edge_index) {
            Some((ni, nj)) => {
                let dist = dist_map.get_mut(&ni).unwrap();
                *dist = edge.len * object.dist;
                queue.push(StateGraph {
                    cost: *dist,
                    node_index: ni,
                });

                let dist = dist_map.get_mut(&nj).unwrap();
                *dist = edge.len * (1.0 - object.dist);
                queue.push(StateGraph {
                    cost: *dist,
                    node_index: nj,
                });
            },
            _ => panic!("Node not found!")
        };

        while let Some(StateGraph { cost, node_index }) = queue.pop() {
            let neighbors = self.graph.neighbors(node_index);

            let dist = dist_map.get(&node_index).unwrap();
            if cost > *dist { continue; }

            for neighbor in neighbors {
                let edge_index = self.graph.find_edge(node_index, neighbor).unwrap();
                let edge = self.graph.edge_weight(edge_index).unwrap();
                let next = cost + edge.len;
                let dist = dist_map.get(&neighbor).unwrap();
                if next < *dist && next < self.max_dist * 2.0 {
                    queue.push(StateGraph {
                        cost: next,
                        node_index: neighbor
                    });

                    for o in edge.objects.borrow().iter() {
                        match object.dominate_pair(o.clone(), &pair) {
                            Domination::Dominator => dominated.push(o.clone()),
                            Domination::Dominated => dominator.push(o.clone()),
                            _ => (),
                        }
                    }

                    let dist = dist_map.get_mut(&neighbor).unwrap();
                    *dist = next;
                }
            }
        }

        (dominator, dominated)
    }

    // insert object to graph and it's edge
    fn insert_object_to_graph(&mut self, object: Rc<Object>) {
        self.objects.insert(object.id, object.clone());
        let edge_index = self.map_edge_index.get(&object.edge_id).unwrap();
        let edge = self.graph.edge_weight(*edge_index).unwrap();
        edge.objects.borrow_mut().push(object.clone());
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

#[derive(Copy, Clone)]
struct StateGraph {
    cost: f32,
    node_index: NodeIndex,
}

impl Ord for StateGraph {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.cost.is_nan() || other.cost.is_nan() {
            panic!("State.cost is NaN!");
        }

        if self.cost < other.cost {
            Ordering::Less
        } else if self.cost > other.cost {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for StateGraph {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.cost.partial_cmp(&self.cost)
    }
}

impl PartialEq for StateGraph {
    fn eq(&self, other: &Self) -> bool {
        if self.cost.is_nan() || other.cost.is_nan() {
            panic!("State.cost is NaN!");
        }
        self.cost == other.cost
    }
}

impl Eq for StateGraph {}

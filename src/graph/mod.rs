use petgraph::graph::NodeIndex;
use petgraph::{Graph as PetGraph, Undirected};
use std::cell::RefCell;
use std::collections::HashMap;

type DimensionIndex = i8;
type ObjectId = i32;
type Scope = HashMap<Vec<DimensionIndex>, Pair>;
type Pair = HashMap<ObjectId, Vec<Range>>;
type NodeId = i32;
type Graph = PetGraph<Node, Edge, Undirected>;

#[allow(dead_code)]
pub struct Structure {
    graph: Graph,
    map_node_index: RefCell<HashMap<NodeId, NodeIndex>>,
}

impl Structure {
    pub fn new(graph: Graph) -> Structure {
        let s = Structure {
            graph: graph,
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
                .into_iter()
                .collect(),
        );
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Node {
    pub id: i32,
    pub lng: f32,
    pub lat: f32,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Edge {
    id: i32,
    len: f32,
    objects: RefCell<Vec<Object>>,
    scope: RefCell<Scope>,
    sky_scope: RefCell<Scope>,
    two_sky_scope: RefCell<Scope>,
    d_sky_scope: RefCell<Scope>,
    k_sky_scope: RefCell<Scope>,
}

impl Edge {
    pub fn new(id: i32, len: f32) -> Edge {
        Edge {
            id,
            len,
            objects: RefCell::new(Vec::new()),
            scope: RefCell::new(HashMap::new()),
            sky_scope: RefCell::new(HashMap::new()),
            two_sky_scope: RefCell::new(HashMap::new()),
            d_sky_scope: RefCell::new(HashMap::new()),
            k_sky_scope: RefCell::new(HashMap::new()),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Object {
    id: ObjectId,
    attr: Vec<f32>,
    dist: f32, // distance from Node I
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Range {
    start: f32,
    end: f32,
}

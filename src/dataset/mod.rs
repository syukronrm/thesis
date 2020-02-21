use csv::ReaderBuilder;
use std::path::{Path, PathBuf};
use std::rc::Rc;

#[derive(Debug)]
pub struct Node {
    pub id: i32,
    pub lng: f32,
    pub lat: f32,
}

#[derive(Debug)]
pub struct Edge {
    pub id: i32,
    pub ni: Rc<Node>,
    pub nj: Rc<Node>,
    pub len: f32,
}

impl Edge {
    #[allow(dead_code)]
    fn new(id: i32, ni: Rc<Node>, nj: Rc<Node>) -> Edge {
        let diff_lng = ni.lng - nj.lng;
        let diff_lat = ni.lat - nj.lat;
        let len = (diff_lng * diff_lng + diff_lat * diff_lat).sqrt();
        Edge { id, ni, nj, len }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Action {
    Insertion,
    Deletion,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct NewObject {
    id: i32,
    attr: Vec<f32>,
    dist: f32,
    edge_id: i32,
    action: Action,
}

impl NewObject {
    pub fn new(id: i32, attr: Vec<f32>, dist: f32, edge_id: i32, action: Action) -> NewObject {
        NewObject {
            id,
            attr,
            dist,
            edge_id,
            action,
        }
    }
}

#[allow(dead_code)]
fn load_edges(dir: &Path, node_file: &str, edge_file: &str) -> Vec<Edge> {
    let node_path = dir.join(node_file);
    let edge_path = dir.join(edge_file);
    let nodes = read_node_csv(&node_path);
    read_edge_csv(&edge_path, &nodes)
}

pub fn read_node_csv(node_path: &PathBuf) -> Vec<Rc<Node>> {
    let mut vec = Vec::new();
    let mut rdr = ReaderBuilder::new()
        .delimiter(b' ')
        .has_headers(false)
        .from_path(node_path)
        .unwrap();

    for result in rdr.records() {
        let record = result.unwrap();
        let id = record
            .get(0)
            .expect("Failed to get index 0")
            .parse::<i32>()
            .expect("Failed to parse Node ID");
        let lng = record
            .get(1)
            .expect("Failed to get index 1")
            .parse::<f32>()
            .expect("Failed to parse lng");
        let lat = record
            .get(2)
            .expect("Failed to get index 2")
            .parse::<f32>()
            .expect("Failed to parse lat");
        vec.push(Rc::new(Node { id, lng, lat }));
    }
    vec.sort_by(|a, b| a.id.partial_cmp(&b.id).unwrap());
    vec
}

#[allow(dead_code, unused_variables)]
pub fn read_edge_csv(edge_path: &PathBuf, nodes: &[Rc<Node>]) -> Vec<Edge> {
    let mut vec = Vec::new();

    let mut rdr = ReaderBuilder::new()
        .delimiter(b' ')
        .has_headers(false)
        .from_path(edge_path)
        .unwrap();

    for result in rdr.records() {
        let record = result.unwrap();
        let id = record
            .get(0)
            .expect("Failed to get index 0")
            .parse::<i32>()
            .expect("Failed to parse Edge ID");
        let ni_id = record
            .get(1)
            .expect("Failed to get index 1")
            .parse::<i32>()
            .expect("Failed to parse node i id");
        let nj_id = record
            .get(2)
            .expect("Failed to get index 2")
            .parse::<i32>()
            .expect("Failed to parse node j id");

        let index_ni = nodes.binary_search_by(|n| n.id.cmp(&ni_id)).unwrap();
        let index_nj = nodes.binary_search_by(|n| n.id.cmp(&nj_id)).unwrap();
        let ni = nodes.get(index_ni).unwrap();
        let nj = nodes.get(index_nj).unwrap();

        vec.push(Edge::new(id, ni.clone(), nj.clone()));
    }

    vec.sort_by(|a, b| a.id.partial_cmp(&b.id).unwrap());
    vec
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::approx_eq;

    #[test]
    fn create_node() {
        let n1 = Node {
            id: 1,
            lng: 1.0,
            lat: 2.0,
        };
        assert_eq!(n1.id, 1);
    }

    #[test]
    fn create_edge() {
        let n1 = Node {
            id: 1,
            lng: 3.0,
            lat: 0.0,
        };
        let n2 = Node {
            id: 2,
            lng: 0.0,
            lat: 4.0,
        };

        let e1 = Edge::new(1, Rc::new(n1), Rc::new(n2));

        assert!(approx_eq!(f32, e1.len, 5.0, ulps = 2));
    }

    #[test]
    fn read_node_file() {
        let project_path = Path::new(env!("CARGO_MANIFEST_DIR"));
        let path = project_path.join("dataset/california/normalized/cal.cnode.txt");
        let nodes = read_node_csv(&path);
        let n0 = &nodes[0];
        assert_eq!(n0.id, 0);
    }

    #[test]
    fn read_edge_file() {
        let project_path = Path::new(env!("CARGO_MANIFEST_DIR"));
        let node_path = project_path.join("dataset/california/normalized/cal.cnode.txt");
        let edge_path = project_path.join("dataset/california/normalized/cal.cedge.txt");
        let nodes = read_node_csv(&node_path);
        let edges = read_edge_csv(&edge_path, &nodes);
        let e0 = &edges[0];
        assert_eq!(e0.ni.id, 0);
        assert_eq!(e0.nj.id, 1);
    }
}

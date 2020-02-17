use csv::ReaderBuilder;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Node {
    pub id: i32,
    pub lng: f32,
    pub lat: f32,
}

#[derive(Debug)]
pub struct Edge<'a> {
    pub id: i32,
    pub ni: &'a Node,
    pub nj: &'a Node,
    pub len: f32,
}

impl<'a> Edge<'a> {
    #[allow(dead_code)]
    fn new(id: i32, ni: &'a Node, nj: &'a Node) -> Edge<'a> {
        let diff_lng = ni.lng - nj.lng;
        let diff_lat = ni.lat - nj.lat;
        let len = (diff_lng * diff_lng + diff_lat * diff_lat).sqrt();
        Edge { id, ni, nj, len }
    }
}

#[allow(dead_code)]
fn reader<'a>(dir: &Path, node_file: &str, edge_file: &str) -> (Vec<Node>, Vec<Edge<'a>>) {
    let node_path = dir.join(node_file);
    let edge_path = dir.join(edge_file);
    let nodes = read_node_csv(&node_path);
    let edges = read_edge_csv(&edge_path, &nodes);
    (nodes, edges)
}

fn read_node_csv(node_path: &PathBuf) -> Vec<Node> {
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
        vec.push(Node { id, lng, lat });
    }
    vec
}

#[allow(dead_code, unused_variables)]
fn read_edge_csv<'a>(edge_path: &PathBuf, nodes: &[Node]) -> Vec<Edge<'a>> {
    let vec = Vec::new();
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

        let e1 = Edge::new(1, &n1, &n2);

        assert!(approx_eq!(f32, e1.len, 5.0, ulps = 2));
    }

    use std::env::current_dir;
    #[test]
    fn read_node_file() {
        let mut path = current_dir().unwrap();
        path.push("dataset/california/normalized/cal.cnode.txt");
        let nodes = read_node_csv(&path);
        let n0 = &nodes[0];
        assert_eq!(n0.id, 0);
    }
}

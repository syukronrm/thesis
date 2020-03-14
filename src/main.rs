mod algo;
mod dataset;
mod structure;

use std::path::Path;

use dataset::Action::*;
use dataset::*;
use structure::*;

use algo::create_initial_graph;

#[allow(clippy::useless_let_if_seq,dead_code)]
fn main2() {
    let project_path = Path::new(env!("CARGO_MANIFEST_DIR"));
    let dataset_dir = project_path.join("dataset/test01");
    let node_csv = "node.txt";
    let edge_csv = "edge.txt";

    println!(
        "Info: use dataset {:?} ({:?} and {:?})",
        dataset_dir, node_csv, edge_csv
    );

    let _graph = create_initial_graph(dataset_dir, node_csv, edge_csv);

    println!("Hello, world!");
}

#[allow(dead_code)]
fn main() {
    use crate::structure::*;

    let mut queries = Multiqueries::new();
    queries.insert(vec![0, 1]);

    let project_path = Path::new(env!("CARGO_MANIFEST_DIR"));
    let dataset_dir = project_path.join("dataset/test01");
    let node_csv = "node.txt";
    let edge_csv = "edge.txt";
    let mut graph = create_initial_graph(dataset_dir, node_csv, edge_csv);

    graph.assign_queries(queries);

    let object_path = project_path.join("dataset/test01/object.txt");
    let objects = read_object_csv(&object_path, 4);
    for (obj, _action) in objects {
        graph.insert(obj);
    }

    println!("{:?}", graph.map_edge_index);
    println!("{:?}", graph.map_node_index);
}

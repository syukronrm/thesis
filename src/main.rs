mod algo;
mod dataset;
mod structure;

use std::path::Path;

use dataset::Action::*;
use dataset::*;

use algo::create_initial_graph;

fn main() {
    let _objects = vec![
        NewObject::new(1, vec![1.0, 8.0, 6.0, 7.0], 10.0, 4, Insertion),
        NewObject::new(2, vec![5.0, 7.0, 1.0, 3.0], 40.0, 5, Insertion),
        NewObject::new(3, vec![5.0, 1.0, 4.0, 5.0], 60.0, 3, Insertion),
        NewObject::new(4, vec![3.0, 4.0, 3.0, 9.0], 20.0, 3, Insertion),
        NewObject::new(5, vec![4.0, 4.0, 4.0, 4.0], 80.0, 3, Insertion),
    ];

    let project_path = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut dataset_dir = project_path.join("dataset/california/normalized");
    let mut node_csv = "cal.cnode.txt";
    let mut edge_csv = "cal.cedge.txt";

    if std::env::var("TEST").is_ok() {
        dataset_dir = project_path.join("dataset/test01");
        node_csv = "node.txt";
        edge_csv = "edge.txt";
    }

    println!(
        "Info: use dataset {:?} ({:?} and {:?})",
        dataset_dir, node_csv, edge_csv
    );

    let _graph = create_initial_graph(dataset_dir, node_csv, edge_csv);

    println!("Hello, world!");
}

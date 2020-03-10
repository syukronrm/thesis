mod algo;
mod dataset;
mod structure;

use std::path::Path;

use dataset::Action::*;
use dataset::*;

use algo::create_initial_graph;

#[allow(clippy::useless_let_if_seq,dead_code)]
fn main2() {
    let _objects = vec![
        NewObject::new(1, vec![1.0, 8.0, 6.0, 7.0], 10.0, 4, Insertion),
        NewObject::new(2, vec![5.0, 7.0, 1.0, 3.0], 40.0, 5, Insertion),
        NewObject::new(3, vec![5.0, 1.0, 4.0, 5.0], 60.0, 3, Insertion),
        NewObject::new(4, vec![3.0, 4.0, 3.0, 9.0], 20.0, 3, Insertion),
        NewObject::new(5, vec![4.0, 4.0, 4.0, 4.0], 80.0, 3, Insertion),
    ];

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
    use crate::algo::voronoi::*;
    use crate::structure::*;
    use std::rc::Rc;

    let project_path = Path::new(env!("CARGO_MANIFEST_DIR"));
    let dataset_dir = project_path.join("dataset/test01");
    let node_csv = "node.txt";
    let edge_csv = "edge.txt";
    let mut graph = create_initial_graph(dataset_dir, node_csv, edge_csv);

    let objects = vec![
        Rc::new(Object {
            id: 1,
            attr: vec![1.0],
            edge_id: 4,
            dist: 0.3535533,
        }),
        Rc::new(Object {
            id: 2,
            attr: vec![1.0],
            edge_id: 1,
            dist: 0.8,
        }),
    ];

    let voronoi = voronoi(&mut graph, &objects, 100.0);
    for (edge, vor) in voronoi.content().into_iter() {
        let edge_id = graph.graph.edge_weight(edge).unwrap().id;
        println!("edge id {:?}", edge_id);
        for v in vor {
            println!("{:?}", v);
        }
    }
    println!("{:?}", graph.map_edge_index);
    println!("{:?}", graph.map_node_index);
}

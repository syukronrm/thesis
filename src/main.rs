mod config;
mod ik;
mod prelude;
mod queries;
mod src;
mod types;

use prelude::*;
use std::sync::Arc;

fn main() {
    let conf: AppConfig = Default::default();

    println!("{}", "main()");
    println!("{:?}", conf);

    construct_new_query();
}

fn construct_new_query() {
    let mut conf = AppConfig::default();
    conf.max_dim = 10;
    conf.max_dist = 100.0;
    conf.path(String::from("dataset/california/normalized"));
    conf.object_path(String::from("dataset/objects/ind.txt"));
    let conf = Arc::new(conf);
    let reader = Reader::new(conf.clone());
    let mut graph = Graph::new(conf.clone());
    let mut result = ResultVoronoi::from_edge_ids(graph.map_edges());

    let mut objects = graph.all_objects();
    objects.sort_by(|a, b| a.id.cmp(&b.id));

    let ks = generate_k(conf.max_dim);

    for object in objects {
        println!("{}", object.id);
        if object.id == 30 {
            break;
        }

        let mut voronoi: Voronoi;

        let mut ks0 = ks.clone();
        let first_k = ks0.remove(0);
        voronoi = Voronoi::initial_voronoi(&mut graph, object.id, first_k);
        voronoi.save_to_result(&mut result, first_k);
        print!("k {}", first_k);

        for k in &ks0 {
            print!(" {}", *k);
            voronoi.continue_voronoi(*k);
            voronoi.save_to_result(&mut result, *k);
        }
        println!(" ");
        graph.clean();
    }
}

fn generate_k(k_max: K) -> Vec<K> {
    let mut ks = Vec::new();

    let lower_bound = (k_max / 2) + 1;
    for i in lower_bound..(k_max + 1) {
        ks.push(i);
    }

    ks
}

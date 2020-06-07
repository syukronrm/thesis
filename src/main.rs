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

    construct();
}

pub fn construct() -> (Graph, ResultVoronoi) {
    let mut conf = AppConfig::default();
    conf.max_dim = 10;
    conf.max_dist = 20.0;
    conf.path(String::from("dataset/california/normalized"));
    conf.object_path(String::from("dataset/objects/ind.txt"));
    let conf = Arc::new(conf);
    let reader = Reader::new(conf.clone());
    let mut graph = Graph::new(conf.clone());
    let queries = Queries::new(reader.read_query_csv());

    let mut result = ResultVoronoi::from_edge_ids(graph.map_edges());

    let mut objects = graph.all_objects();
    objects.sort_by(|a, b| a.id.cmp(&b.id));
    for object in objects {
        println!("{}", object.id);
        if object.id == 30 {
            break;
        }

        for g in queries.iter() {
            let mut g = g.clone();
            let mut voronoi: Voronoi;
            if let Some(q) = g.pop_first() {
                voronoi = Voronoi::initial_voronoi(&mut graph, object.id, q.k);
                voronoi.save_to_result(&mut result, q.k);
            } else {
                continue;
            }

            for q in g.iter() {
                voronoi.continue_voronoi(q.k);
                voronoi.save_to_result(&mut result, q.k);
            }
        }
        graph.clean();
    }

    (graph, result)
}
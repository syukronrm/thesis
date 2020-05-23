use crate::prelude::*;
use std::sync::Arc;

#[allow(dead_code)]
pub fn main() {
    let conf = Arc::new(AppConfig::default());
    let reader = Reader::new(conf.clone());
    let mut graph = Graph::new(conf.clone());
    let queries = Queries::new(reader.read_query_csv());

    // let result = ResultVoronoi::from_edge_ids(graph.all_edge_ids());

    for object in graph.all_objects() {
        for g in queries.iter() {
            let mut g = g.clone();
            let mut voronoi: Voronoi;
            if let Some(q) = g.pop_first() {
                voronoi = Voronoi::initial_voronoi(&mut graph, object.id, q.k);
            } else {
                continue;
            }

            for q in g.iter() {
                voronoi.continue_voronoi(q.k);
            }
        }
    }
}

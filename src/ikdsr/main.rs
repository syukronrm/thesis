use crate::prelude::*;
use std::sync::Arc;

#[allow(dead_code)]
pub fn main() {
    let conf = Arc::new(AppConfig::default());
    let reader = Reader::new(conf.clone());
    let mut graph = Graph::new(conf.clone());
    let queries = Queries::new(reader.read_query_csv());

    let mut result = Result::from_edge_ids(graph.all_edge_ids());

    for object in graph.all_objects() {
        for g in queries.iter() {
            let mut voronoi: Voronoi;
            for (index, q) in g.iter() {
                if index == 0 {
                    voronoi = Voronoi::initial_voronoi(&mut graph, object.id, q.k);
                    // TODO: save to `result`
                } else {
                }
            }
        }
    }
}

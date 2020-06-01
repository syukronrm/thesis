pub mod algo;
pub mod structure;
pub mod types;

use crate::prelude::*;
use std::sync::Arc;

#[allow(dead_code)]
pub fn construct() {
    let conf = Arc::new(AppConfig::default());
    let reader = Reader::new(conf.clone());
    let mut graph = Graph::new(conf.clone());
    let queries = Queries::new(reader.read_query_csv());

    let mut result = ResultVoronoi::from_edge_ids(graph.map_edges());

    for object in graph.all_objects() {
        for g in queries.iter() {
            let mut g = g.clone();
            let mut voronoi: Voronoi;
            if let Some(q) = g.pop_first() {
                voronoi = Voronoi::initial_voronoi(&mut graph, object.id, q.k);
                voronoi.save_to_result(&mut result);
            } else {
                continue;
            }

            for q in g.iter() {
                voronoi.continue_voronoi(q.k);
                voronoi.save_to_result(&mut result);
            }
        }
        graph.clean();
    }
}

#[allow(dead_code)]
pub fn insertion() {
    let conf = Arc::new(AppConfig::default());
    let reader = Reader::new(conf.clone());
    let mut graph = Graph::new_empty_object(conf.clone());
    let queries = Queries::new(reader.read_query_csv());

    let mut result = ResultVoronoi::from_edge_ids(graph.map_edges());
    let objects = reader.read_object_csv();
    for object in objects {
        graph.insert_object(object.clone());
        let dom_traverse = DomTraverse::dominate_dominated_by(&mut graph, object.clone());
        let dominated_by_objects = dom_traverse.map_dominate_objects();

        for g in queries.iter() {
            let mut g0 = g.clone();
            let mut voronoi: Voronoi;
            if let Some(q) = g0.pop_first() {
                voronoi = Voronoi::initial_voronoi(&mut graph, object.id, q.k);
                voronoi.save_to_result(&mut result);
            } else {
                continue;
            }

            for q in g0.iter() {
                voronoi.continue_voronoi(q.k);
                voronoi.save_to_result(&mut result);
            }

            // TODO: compute dominated objects by `object`
            for (dominated, k) in dominated_by_objects.clone() {
                let mut g1 = g.clone();
                g1.remove_less_k(k);
                result.remove(dominated, k);
                let mut g2 = g1.clone();
                let mut voronoi: Voronoi;
                if let Some(q) = g2.pop_first() {
                    voronoi = Voronoi::initial_voronoi(&mut graph, dominated, q.k);
                    voronoi.save_to_result(&mut result);
                } else {
                    continue;
                }

                for q in g2.iter() {
                    voronoi.continue_voronoi(q.k);
                    voronoi.save_to_result(&mut result);
                }
                graph.clean();
            }
        }
    }
    println!("{:#?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main_test() {
        construct();
    }

    #[test]
    fn main_test_california() {
        let mut conf = AppConfig::default();
        // conf.path(String::from("dataset/california/normalized"));
        let conf = Arc::new(conf);
        let reader = Reader::new(conf.clone());
        let mut graph = Graph::new(conf.clone());
        let queries = Queries::new(reader.read_query_csv());

        let mut result = ResultVoronoi::from_edge_ids(graph.map_edges());

        for object in graph.all_objects() {
            for g in queries.iter() {
                let mut g = g.clone();
                let mut voronoi: Voronoi;
                if let Some(q) = g.pop_first() {
                    voronoi = Voronoi::initial_voronoi(&mut graph, object.id, q.k);
                    voronoi.save_to_result(&mut result);
                } else {
                    continue;
                }

                for q in g.iter() {
                    voronoi.continue_voronoi(q.k);
                    voronoi.save_to_result(&mut result);
                }
            }
            graph.clean();
        }
        println!("{:#?}", result);
    }

    #[test]
    fn test_insertion() {
        insertion();
    }
}

pub mod algo;
pub mod structure;
pub mod types;

use crate::prelude::*;
use std::sync::Arc;

#[allow(dead_code)]
pub fn construct() -> Graph {
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

    graph
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
        let dominate_objects = dom_traverse.map_dominate_objects();

        for g in queries.iter() {
            let mut g0 = g.clone();
            let mut voronoi: Voronoi;
            if let Some(q) = g0.pop_first() {
                voronoi = Voronoi::initial_voronoi(&mut graph, object.id, q.k);
                voronoi.save_to_result(&mut result, q.k);
            } else {
                continue;
            }

            for q in g0.iter() {
                voronoi.continue_voronoi(q.k);
                voronoi.save_to_result(&mut result, q.k);
            }

            // TODO: compute dominated objects by `object`
            for (dominate_object, k) in dominate_objects.clone() {
                let mut g1 = g.clone();
                g1.remove_less_k(k);

                let mut g2 = g1.clone();
                let mut voronoi: Voronoi;
                if let Some(q) = g2.pop_first() {
                    result.remove(dominate_object + 100000, q.k);
                    voronoi = Voronoi::initial_voronoi(&mut graph, dominate_object, q.k);
                    voronoi.save_to_result(&mut result, q.k);
                } else {
                    continue;
                }

                for q in g2.iter() {
                    result.remove(dominate_object + 100000, q.k);
                    voronoi.continue_voronoi(q.k);
                    voronoi.save_to_result(&mut result, q.k);
                }
                graph.clean();
            }
        }
    }
    println!("{:#?}", result);
}

#[allow(dead_code)]
fn deletion() {
    let conf = Arc::new(AppConfig::default());
    let reader = Reader::new(conf.clone());
    let mut graph = construct();
    let queries = Queries::new(reader.read_query_csv());

    let mut result = ResultVoronoi::from_edge_ids(graph.map_edges());

    let deleted_objects = vec![1, 3];
    for object_id in deleted_objects {
        let object = graph.object(object_id);
        let dom_traverse = DomTraverse::dominate_dominated_by(&mut graph, object);
        let dominate_objects = dom_traverse.map_dominate_objects();
        graph.remove_object(object_id);

        for (dominate_object, _) in &dominate_objects {
            result.remove_all(*dominate_object + 100000);
        }

        for g in queries.iter() {
            for (dominate_object, k) in dominate_objects.clone() {
                let mut g1 = g.clone();
                g1.remove_less_k(k);
                result.remove(dominate_object + 100000, k);
                let mut g2 = g1.clone();
                let mut voronoi: Voronoi;
                if let Some(q) = g2.pop_first() {
                    result.remove(dominate_object + 100000, q.k);
                    voronoi = Voronoi::initial_voronoi(&mut graph, dominate_object, q.k);
                    voronoi.save_to_result(&mut result, q.k);
                } else {
                    continue;
                }

                for q in g2.iter() {
                    result.remove(dominate_object + 100000, q.k);
                    voronoi.continue_voronoi(q.k);
                    voronoi.save_to_result(&mut result, q.k);
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
        println!("{:#?}", result);
    }

    #[test]
    fn test_insertion() {
        insertion();
    }

    #[test]
    fn test_deletion() {
        deletion();
    }
}

use std::collections::{BTreeSet, HashMap};
use std::rc::Rc;

use petgraph::graph::NodeIndex;

use crate::structure::edge::{Object, Range};
use crate::structure::voronoi::state::State;
use crate::structure::voronoi::Voronoi;
use crate::structure::*;

type ObjectId = i32;
type Queue = BTreeSet<State>;

struct VisitedNodes(HashMap<NodeIndex, (f32, ObjectId)>);

impl VisitedNodes {
    fn new() -> VisitedNodes {
        VisitedNodes({ HashMap::new() })
    }

    #[allow(dead_code)]
    fn contains(self, node_id: NodeIndex) -> Option<(f32, ObjectId)> {
        if let Some((dist, object_id)) = self.0.get(&node_id) {
            Some((*dist, *object_id))
        } else {
            None
        }
    }

    fn insert(&mut self, k: NodeIndex, v: (f32, ObjectId)) {
        self.0.insert(k, v);
    }

    fn visited(&self, centroid_id: i32, node_id: NodeIndex) -> bool {
        if let Some((_, object_id)) = self.0.get(&node_id) {
            object_id == &centroid_id
        } else {
            false
        }
    }

    fn replace(&mut self, node_id: NodeIndex, dist: f32, centroid_id: ObjectId) {
        if let Some(x) = self.0.get_mut(&node_id) {
            *x = (dist, centroid_id);
        } else {
            self.0.insert(node_id, (dist, centroid_id));
        }
    }
}

fn enqueue_neighbors(queue: &mut Queue, g: &Graph, centroids: &[Rc<Object>]) {
    for c in centroids {
        let edge_index = g.edge_index(c.edge_id);
        let (node_index_m, node_index_n) = g.graph.edge_endpoints(edge_index).unwrap();
        let edge = g.graph.edge_weight(edge_index).unwrap();
        let node_n = g.graph.node_weight(node_index_n).unwrap();
        if edge.ni == node_n.id {
            let dist = edge.len * c.dist;
            println!(
                "Enqueue 1 {:?} from centroid {:?} distance {:?}",
                node_index_n, c.id, dist
            );
            queue.insert(State::new(node_index_n, c.id, dist));

            let dist = edge.len * (1.0 - c.dist);
            println!(
                "Enqueue 2 {:?} from centroid {:?} distance {:?}",
                node_index_m, c.id, dist
            );
            queue.insert(State::new(node_index_m, c.id, dist));
        } else {
            let dist = edge.len * (1.0 - c.dist);
            println!(
                "Enqueue 3 {:?} from centroid {:?} distance {:?}",
                node_index_n, c.id, dist
            );
            queue.insert(State::new(node_index_n, c.id, dist));

            let dist = edge.len * c.dist;
            println!(
                "Enqueue 4 {:?} from centroid {:?} distance {:?}",
                node_index_m, c.id, dist
            );
            queue.insert(State::new(node_index_m, c.id, dist));
        }
    }
}

#[allow(dead_code, unused_variables, unused_mut)]
fn compute_voronoi(
    g: &Graph,
    src_centroid_id: ObjectId,
    centroid_ids: Vec<ObjectId>,
    max_distance: f32,
    dimensions: Vec<i8>,
) -> Voronoi {
    let mut voronoi = Voronoi::new();
    let mut visited_nodes = VisitedNodes::new();
    let mut queue: Queue = BTreeSet::new();
    let graph = &g.graph;
    let objects = g.objects.borrow();
    let centroids = g.get_objects(centroid_ids);

    enqueue_neighbors(&mut queue, g, &centroids);

    let centroid = objects.get(&src_centroid_id).unwrap();

    while !queue.is_empty() {
        let state = queue.iter().next().unwrap().clone();
        assert_eq!(queue.remove(&state), true);
        println!("POP {:?}", state);
        if state.dist > max_distance {
            println!("Continue");
            continue;
        }

        let State {
            node_index: node_index_n,
            centroid_id,
            dist: dist_n,
        } = state;
        let neighbors = graph.neighbors(node_index_n);
        for node_index_m in neighbors {
            println!("  Neighbor {:?}", node_index_m);
            // if visited_nodes.visited(centroid_id, node_index_m) {
            //     println!("    Already visited by its' centroid id!");
            //     continue;
            // }

            let edge_index_m = graph.find_edge(node_index_n, node_index_m).unwrap();
            let edge = graph.edge_weight(edge_index_m).unwrap();
            let dist_m = dist_n + edge.len;
            println!("  dist_n {:?}", dist_n);
            println!("  dist_m {:?}", dist_m);
            if let Some((dist, existing_centroid_id)) = visited_nodes.0.get(&node_index_m).cloned()
            {
                println!(
                    "    node_index_m is already visited by {:?} dist {:?}",
                    existing_centroid_id, dist
                );
                if existing_centroid_id == centroid_id {
                    println!(
                        "      continue existing_centroid_id {:?}",
                        existing_centroid_id
                    );
                    continue;
                }

                if dist_m < dist {
                    println!("      dist_m {} < dist {}", dist_m, dist);
                    visited_nodes.replace(node_index_m, dist_m, centroid_id);
                    queue.remove(&State::new(node_index_m, existing_centroid_id, dist));
                } else if centroid_id == src_centroid_id {
                    let center = (dist_n + dist + edge.len) / 2.0;
                    let dist_from_n = center - dist_n;
                    voronoi.insert(edge_index_m, Range::new(0.0, dist_from_n, centroid.clone()));
                }
            } else {
                println!("    node_index_m is not visited");
                if dist_m < max_distance {
                    let state = State::new(node_index_m, centroid_id, dist_m);
                    println!("      enqueue {:?}", state);
                    queue.insert(state);
                    println!(
                        "      set as visited {:?} dist_m {:?} from centroid {:?}",
                        node_index_m, dist_m, centroid_id
                    );
                    visited_nodes.insert(node_index_m, (dist_m, centroid_id));
                }

                if centroid_id == src_centroid_id {
                    let node_n = graph.node_weight(node_index_n).unwrap();
                    let mut start;
                    let mut end;
                    println!(
                        "      if edge.ni == node_n.id {:?} {:?}",
                        edge.ni, node_n.id
                    );
                    if edge.ni == node_n.id {
                        println!("        edge.ni == node_n.id");
                        start = 0.0;
                        if dist_n + edge.len > max_distance {
                            end = edge.len;
                        } else {
                            end = max_distance - dist_n;
                        }
                    } else {
                        println!("        else");
                        start = edge.len;
                        if dist_n + edge.len > max_distance {
                            end = 0.0;
                        } else {
                            end = edge.len - (max_distance - dist_n);
                        }
                    }

                    let range = Range::new(start, end, centroid.clone());
                    println!(
                        "      insert voronoi edge_index_m {:?} range {:?}",
                        edge_index_m, range
                    );
                    voronoi.insert(edge_index_m, range);
                }
            }
        }
    }

    voronoi
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use std::rc::Rc;

    #[test]
    fn graph() {
        use crate::structure::*;

        let project_path = Path::new(env!("CARGO_MANIFEST_DIR"));
        let dataset_dir = project_path.join("dataset/test01");
        let node_csv = "node.txt";
        let edge_csv = "edge.txt";

        let graph = crate::algo::create_initial_graph(dataset_dir, node_csv, edge_csv);
        let objects = vec![
            Rc::new(Object::new(1, vec![1.0, 2.0], 0.3535, 4)), // dist 20
            Rc::new(Object::new(4, vec![1.0, 2.0], 0.2, 3)),
        ];
        graph.insert_objects(objects);
        println!("Edge {:?}", graph.map_edge_index);
        println!("Node {:?}", graph.map_node_index);
        let voronoi = compute_voronoi(&graph, 1, vec![1, 4], 100.0, vec![1, 2]);

        voronoi.print();

        println!("Edge {:?}", graph.map_edge_index);
    }
}

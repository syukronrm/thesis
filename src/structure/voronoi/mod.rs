use std::collections::HashMap;

use petgraph::graph::EdgeIndex;

use super::edge::Range;

pub mod state;

#[allow(dead_code)]
pub struct Voronoi(HashMap<EdgeIndex, Vec<Range>>);

/// find voronoi:
///   // input: &graph, source_centroid, centroid_ids, max_distance
///   // outout: voronoi
///   mut voronoi = hashmap<_>
///   mut visited = hashmap<nodeid, (dist, centroid_id, edge_pred)>
///   let q = binaryheap<state>
///   initial enqueue 2 neigbor of centroids and its distance
///   while q not empty do:
///     n = q.dequeue
///     if n < max_distance:
///       continue
///     for each edge e from n to m in n.neighbors() and m not visited by n.centroid_id do
///       dist_m = n.distance + e.len
///       if m is not visited by any centroid:
///         if dist_m < max_distance:
///           q.enqueue(state(m, q.centroid_id, dist_m))
///           visited.insert(m.id, (dist_m, q.centroid_id))
///           if q.centroid_id == source_centroid:
///             insert e to voronoi till max_distance
///       else if m is visited by another centroid:
///         if dist_m < existing distance:
///           replace visited with a new arrived q
///           replace queued node with a new arrived q
///         else
///           if q.centroid_id == source_centroid:
///             compute voronoi in e with node visited.node and q.node

impl Voronoi {
    #[allow(dead_code)]
    pub fn new() -> Voronoi {
        Voronoi(HashMap::new())
    }

    pub fn insert(&mut self, edge_index: EdgeIndex, range: Range) {
        if let Some(ranges) = self.0.get_mut(&edge_index) {
            ranges.push(range);
        } else {
            self.0.insert(edge_index, vec![range]);
        }
    }
}

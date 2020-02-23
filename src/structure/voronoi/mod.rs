use std::collections::HashMap;

use super::edge::Range;

pub mod state;

type EdgeId = i32;

#[allow(dead_code)]
struct Voronoi(HashMap<EdgeId, Vec<Range>>);

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
///     for each edge e from n to m in n.neighbors() not visited by q.centroid_id do
///       dist = n.distance + e.len
///       if m is not visited:
///         if dist < max_distance:
///           q.enqueue(state(m, q.centroid_id, dist))
///           visited.insert(m.id, (dist, q.centroid_id))
///         if q.centroid_id == source_centroid:
///           insert e to voronoi till max_distance
///       else if m is visited by another centroid:
///         if dist < existing distance:
///           replace visited with a new arrived q
///           replace queued node with a new arrived q
///         else
///           if q.centroid_id == source_centroid:
///             compute voronoi in e with node visited.node and q.node

impl Voronoi {
    #[allow(dead_code)]
    fn new() -> Voronoi {
        Voronoi(HashMap::new())
    }
}

use crate::prelude::*;
use ordered_float::OrderedFloat as OF;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::sync::Arc;

#[derive(Debug)]
pub struct ResultVoronoi {
    inner: HashMap<EdgeId, HashMap<K, EdgeResult>>,
    edges: HashMap<EdgeId, Arc<DataEdge>>,
}

impl ResultVoronoi {
    pub fn from_edge_ids(edges: HashMap<EdgeId, Arc<DataEdge>>) -> Self {
        let result = ResultVoronoi {
            inner: HashMap::new(),
            edges,
        };

        result
    }

    /// insert voronoi scope in `edge_id`
    pub fn insert(&mut self, k: K, edge_id: EdgeId, ranges: Vec<Range>) {
        let edge = self.edges.get(&edge_id).unwrap();

        if let Some(k_edge_result) = self.inner.get_mut(&edge_id) {
            if let Some(edge_result) = k_edge_result.get_mut(&k) {
                for r in ranges {
                    edge_result.insert(r);
                }
            } else {
                let mut edge_result = EdgeResult::new(edge.len);
                for r in ranges {
                    edge_result.insert(r);
                }
                k_edge_result.insert(k, edge_result);
            }
        } else {
            let mut hash = HashMap::new();
            let mut edge_result = EdgeResult::new(edge.len);
            for r in ranges {
                edge_result.insert(r);
            }
            hash.insert(k, edge_result);
            self.inner.insert(edge_id, hash);
        }
    }

    pub fn remove(&mut self, object_id: ObjectId, k: K) {
        for (_edge_id, k_edge_result) in &mut self.inner {
            if let Some(edge_result) = k_edge_result.get_mut(&k) {
                edge_result.remove(object_id);
            }
        }
    }

    pub fn remove_all(&mut self, object_id: ObjectId) {
        for (_edge_id, k_edge_result) in &mut self.inner {
            for (_k, edge_result) in k_edge_result {
                edge_result.remove(object_id);
            }
        }
    }
}

#[derive(Debug)]
struct EdgeResult {
    ranges: Vec<Range>,
    edge_len: f32,
    inner: BTreeMap<OF<f32>, Vec<ObjectId>>,
}

impl EdgeResult {
    fn new(edge_len: f32) -> Self {
        let mut inner = BTreeMap::new();
        inner.insert(OF(edge_len), vec![]);

        EdgeResult {
            ranges: Vec::new(),
            edge_len,
            inner,
        }
    }

    fn insert(&mut self, range: Range) {
        self.ranges.push(range.clone());
        let object_id = range.centroid_id;
        let start = OF(range.start);
        let end = OF(range.end);
        let mut inner_clone = self.inner.clone();
        let existing_ranges = self.inner.range(start..);
        let mut range_iter = existing_ranges.into_iter().peekable();

        let mut first = true;
        loop {
            let next = range_iter.next();

            if let Some((k, v)) = next {
                if *k < end {
                    if first {
                        inner_clone.insert(start, v.clone());
                        first = false;
                    }

                    let object_ids = inner_clone.get_mut(k).unwrap();
                    object_ids.push(object_id);
                } else {
                    let mut object_ids = inner_clone.get(k).unwrap().clone();
                    object_ids.push(object_id);

                    inner_clone.insert(end, object_ids);
                    break;
                }
            } else {
                break;
            }
        }

        self.inner = inner_clone;
    }

    fn remove(&mut self, object_id: CentroidId) {
        self.ranges.retain(|r| r.centroid_id != object_id);
        let mut deleted_dist = HashSet::new();
        for (dist, vec_object_id) in &mut self.inner {
            let mut is_deleted = false;
            vec_object_id.retain(|o| {
                if o == &object_id {
                    is_deleted = true;
                    false
                } else {
                    true
                }
            });
            if is_deleted {
                deleted_dist.insert(*dist);
            }
            vec_object_id.sort();
        }
        let inner_clone = self.inner.clone();
        let mut inner_clone_iter = inner_clone.iter().peekable();
        loop {
            let dist = inner_clone_iter.next();
            let next_dist = inner_clone_iter.peek();
            if dist.is_none() || next_dist.is_none() {
                break;
            }
            let (dist, vec_object_id) = dist.unwrap();
            let (_n_dist, n_vec_object_id) = next_dist.unwrap();
            if vec_object_id == *n_vec_object_id {
                self.inner.remove(dist);
            }
        }
    }
}

impl Default for EdgeResult {
    fn default() -> Self {
        EdgeResult {
            ranges: Vec::new(),
            edge_len: 0.0,
            inner: BTreeMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn edge_result() {
        let mut edge_result = EdgeResult::new(10.0);
        let ranges = vec![
            Range {
                start: 0.0,
                end: 4.0,
                centroid_id: 1,
            },
            Range {
                start: 2.0,
                end: 4.0,
                centroid_id: 2,
            },
            Range {
                start: 1.0,
                end: 7.0,
                centroid_id: 3,
            },
            Range {
                start: 3.0,
                end: 9.0,
                centroid_id: 4,
            },
        ];

        for range in ranges {
            edge_result.insert(range);
        }

        println!("{:#?}", edge_result);
        assert_eq!(edge_result.inner.len(), 6);

        edge_result.remove(3);
        println!("{:#?}", edge_result);
        assert_eq!(edge_result.ranges.len(), 3);
        assert_eq!(edge_result.inner.len(), 4);
    }
}

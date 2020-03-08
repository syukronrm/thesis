use std::collections::HashMap;

use petgraph::graph::EdgeIndex;

use super::edge::Range;

pub mod state;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Voronoi(HashMap<EdgeIndex, Vec<Range>>);

impl Voronoi {
    #[allow(dead_code)]
    pub fn new() -> Voronoi {
        Voronoi(HashMap::new())
    }

    #[allow(dead_code)]
    pub fn insert(&mut self, edge_index: EdgeIndex, range: Range) {
        if let Some(ranges) = self.0.get_mut(&edge_index) {
            ranges.push(range);
        } else {
            self.0.insert(edge_index, vec![range]);
        }
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        println!("Voronoi");
        for (key, val) in self.0.iter() {
            println!("EdgeIndex: {:?} val: {:?}", key, val);
        }
    }
}

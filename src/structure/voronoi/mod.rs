use std::collections::HashMap;

use super::edge::Range;

pub mod state;

type EdgeId = i32;

#[allow(dead_code)]
struct Voronoi(HashMap<EdgeId, Vec<Range>>);


impl Voronoi {
    #[allow(dead_code)]
    fn new() -> Voronoi {
        Voronoi(HashMap::new())
    }
}

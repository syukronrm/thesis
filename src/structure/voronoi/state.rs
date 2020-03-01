use petgraph::graph::NodeIndex;
use std::cmp::Ordering;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct State {
    pub node_index: NodeIndex,
    pub centroid_id: NodeIndex,
    pub dist: f32,
}

impl State {
    pub fn new(node_index: NodeIndex, centroid_id: NodeIndex, dist: f32) -> State {
        State {
            node_index,
            centroid_id,
            dist,
        }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.dist.is_nan() || other.dist.is_nan() {
            panic!("State.dist is NaN!");
        }

        if self.dist < other.dist {
            Ordering::Less
        } else if self.dist > other.dist {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.dist.partial_cmp(&self.dist)
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        if self.dist.is_nan() || other.dist.is_nan() {
            panic!("State.dist is NaN!");
        }
        self.dist == other.dist
    }
}

impl Eq for State {}

use std::cmp::Ordering;

type NodeId = i32;
type CentroidId = i32;

#[allow(dead_code)]
pub struct State {
    node_id: NodeId,
    centroid_id: CentroidId,
    dist: f32,
}

impl State {
    pub fn new(node_id: NodeId, centroid_id: CentroidId, dist: f32) -> State {
        State {
            node_id,
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

        if other.dist < self.dist {
            Ordering::Less
        } else if other.dist > self.dist {
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

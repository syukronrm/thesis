use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

type DimensionIndex = i8;
type ObjectId = i32;
type Scope = HashMap<Vec<DimensionIndex>, Pair>;
type Pair = HashMap<ObjectId, Vec<Range>>;

#[derive(Debug)]
pub struct Object {
    id: ObjectId,
    attr: Vec<f32>,
    dist: f32, // distance from Node I
}

#[derive(Debug)]
pub struct Range {
    start: f32,
    end: f32,
    object: Rc<Object>,
}

#[derive(Debug)]
pub struct Edge {
    id: i32,
    len: f32,
    objects: RefCell<Vec<Object>>,
    scope: RefCell<Scope>,
    sky_scope: RefCell<Scope>,
    two_sky_scope: RefCell<Scope>,
    d_sky_scope: RefCell<Scope>,
    k_sky_scope: RefCell<Scope>,
}

macro_rules! create_insert_scope {
    ($func_name:ident, $field:ident) => {
        #[allow(dead_code)]
        pub fn $func_name(&self, dimensions: Vec<DimensionIndex>, range: Range) {
            fn scope<'a>(edge: &'a Edge) -> &'a RefCell<Scope> {
                &edge.$field
            }
            self.insert(scope, dimensions, range);
        }
    };
}

impl Edge {
    pub fn new(id: i32, len: f32) -> Edge {
        Edge {
            id,
            len,
            objects: RefCell::new(Vec::new()),
            scope: RefCell::new(HashMap::new()),
            sky_scope: RefCell::new(HashMap::new()),
            two_sky_scope: RefCell::new(HashMap::new()),
            d_sky_scope: RefCell::new(HashMap::new()),
            k_sky_scope: RefCell::new(HashMap::new()),
        }
    }

    fn insert(
        &self,
        f: fn(&Edge) -> &RefCell<Scope>,
        dimensions: Vec<DimensionIndex>,
        range: Range,
    ) {
        let mut scope = f(self).borrow_mut();
        if let Some(pair) = scope.get_mut(&dimensions) {
            if let Some(ranges) = pair.get_mut(&range.object.id) {
                ranges.push(range);
            } else {
                pair.insert(range.object.id, vec![range]);
            }
        } else {
            let new_pair = Edge::new_pair(range);
            scope.insert(dimensions, new_pair);
        };
    }

    fn new_pair(range: Range) -> Pair {
        let mut pair = HashMap::new();
        pair.insert(range.object.id, vec![range]);
        pair
    }

    create_insert_scope!(insert_scope, scope);
    create_insert_scope!(insert_sky_scope, sky_scope);
    create_insert_scope!(insert_two_sky_scope, two_sky_scope);
    create_insert_scope!(insert_d_sky_scope, d_sky_scope);
    create_insert_scope!(insert_k_sky_scope, k_sky_scope);
}

#[cfg(test)]
mod test {
    use super::*;
    use float_cmp::approx_eq;

    #[test]
    fn insert_scope_test() {
        let edge = Edge::new(1, 100.0);
        let object = Object {
            id: 1,
            attr: vec![1.0, 2.0],
            dist: 10.0,
        };
        let range = Range {
            start: 1.0,
            end: 1.0,
            object: Rc::new(object),
        };
        edge.insert_scope(vec![1, 2], range);

        let object_scope = edge.scope.borrow();
        let scope = object_scope.get(&vec![1, 2]).unwrap();
        let ranges = scope.get(&1).unwrap();
        let range = &ranges[0];
        assert!(approx_eq!(f32, range.start, 1.0, ulps = 2));
    }

    #[test]
    fn new_pair_test() {
        let object = Object {
            id: 1,
            attr: vec![1.0, 2.0],
            dist: 10.0,
        };
        let pair = Edge::new_pair(Range {
            start: 1.0,
            end: 2.0,
            object: Rc::new(object),
        });
        let ranges = pair.get(&1).unwrap();
        assert!(approx_eq!(f32, ranges[0].start, 1.0, ulps = 2));
    }
}

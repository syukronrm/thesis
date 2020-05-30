use crate::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;

pub struct Queries {
    inner: Vec<Group>,
}

impl Queries {
    pub fn new(queries: Vec<Arc<Query>>) -> Self {
        let mut groups: HashMap<Vec<DimensionIndex>, Group> = HashMap::new();
        for q in queries {
            if let Some(group) = groups.get_mut(&q.dimensions) {
                group.queries.push(q);
            } else {
                let dim = q.dimensions.clone();
                let new_group = Group {
                    dimensions: q.dimensions.clone(),
                    queries: vec![q],
                };
                groups.insert(dim, new_group);
            }
        }
        let mut inner = Vec::new();
        for (_, group) in &mut groups {
            group.queries.sort_by(|a, b| a.k.partial_cmp(&b.k).unwrap());
            inner.push(group.clone());
        }
        Queries { inner }
    }

    #[cfg(test)]
    fn length(&self) -> usize {
        self.inner.len()
    }

    pub fn iter(&self) -> QueryIterator {
        QueryIterator::new(&self.inner)
    }
}

pub struct QueryIterator<'a> {
    queries: &'a Vec<Group>,
    index: usize,
}

impl<'a> QueryIterator<'a> {
    fn new(queries: &'a Vec<Group>) -> Self {
        QueryIterator {
            queries: queries,
            index: 0,
        }
    }
}

impl<'a> Iterator for QueryIterator<'a> {
    type Item = &'a Group;

    fn next(&mut self) -> Option<Self::Item> {
        let g = self.queries.get(self.index);
        self.index += 1;
        g
    }
}

#[derive(Clone)]
pub struct Group {
    dimensions: Vec<DimensionIndex>,
    queries: Vec<Arc<Query>>,
}

impl Group {
    pub fn iter(&self) -> GroupIterator {
        GroupIterator::new(&self.queries)
    }

    pub fn pop_first(&mut self) -> Option<Arc<Query>> {
        if self.queries.len() != 0 {
            Some(self.queries.remove(0))
        } else {
            None
        }
    }

    pub fn remove_less_k(&mut self, k: K) {
        self.queries.retain(|q| q.k >= k);
    }
}

pub struct GroupIterator<'a> {
    queries: &'a Vec<Arc<Query>>,
    index: usize,
}

impl<'a> GroupIterator<'a> {
    fn new(queries: &'a Vec<Arc<Query>>) -> Self {
        GroupIterator { queries, index: 0 }
    }
}

impl<'a> Iterator for GroupIterator<'a> {
    type Item = &'a Arc<Query>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(q) = self.queries.get(self.index) {
            self.index += 1;
            Some(q)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn queries_new() {
        let conf: AppConfig = Default::default();
        let conf = Arc::new(conf);
        let reader = Reader::new(conf);
        let queries = reader.read_query_csv();

        let queries = Queries::new(queries);
        assert_eq!(queries.length(), 1);

        let mut is_exists = false;
        for q in queries.iter() {
            if q.dimensions == vec![1, 2, 3, 4] {
                assert_eq!(q.queries.len(), 2);
                is_exists = true;
            }
        }
        assert_eq!(is_exists, true);
    }
}

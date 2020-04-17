use crate::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;

pub struct Queries {
    inner: Vec<Group>,
}

#[derive(Clone)]
pub struct Group {
    dimensions: Vec<DimensionIndex>,
    queries: Vec<Arc<Query>>,
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

    fn length(&self) -> usize {
        self.inner.len()
    }
}

impl IntoIterator for Queries {
    type Item = Group;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl IntoIterator for Group {
    type Item = Arc<Query>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.queries.into_iter()
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
        assert_eq!(queries.length(), 2);

        let mut is_exists = false;
        for q in queries {
            if q.dimensions == vec![1, 2, 3, 4] {
                assert_eq!(q.queries.len(), 2);
                is_exists = true;
            }
        }
        assert_eq!(is_exists, true);
    }
}

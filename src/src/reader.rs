use csv::ReaderBuilder;
use std::sync::Arc;

use crate::prelude::*;

/// Centralized reader of all datasets
pub struct Reader {
    config: Arc<AppConfig>,
}

impl Reader {
    /// Create new Reader
    pub fn new(config: Arc<AppConfig>) -> Self {
        Reader { config }
    }

    /// Read object from CSV file
    pub fn read_object_csv(&self) -> Vec<Arc<DataObject>> {
        let config = self.config.clone();
        let mut vec = Vec::new();

        let mut rdr = ReaderBuilder::new()
            .delimiter(b' ')
            .from_path(config.paths.object_path.as_path())
            .unwrap();

        for result in rdr.records() {
            let record = result.unwrap();
            let action = record.get(0).unwrap().parse::<i32>().unwrap();
            let id = record.get(1).unwrap().parse::<ObjectId>().unwrap();
            let edge_id = record.get(2).unwrap().parse::<EdgeId>().unwrap();
            let dist = record.get(3).unwrap().parse::<f32>().unwrap();
            let mut attr = Vec::new();
            for i in 0..self.config.max_dim {
                let val = record.get((4 + i).into()).unwrap().parse::<f32>().unwrap();
                attr.push(val);
            }
            let action = if action == 1 {
                Action::Insertion
            } else {
                Action::Deletion
            };
            let new_object = Arc::new(DataObject {
                id,
                attr,
                dist,
                edge_id,
                action,
            });
            vec.push(new_object);
        }

        vec
    }

    pub fn read_node_csv(&self) -> Vec<Arc<DataNode>> {
        let mut vec = Vec::new();
        let mut rdr = ReaderBuilder::new()
            .delimiter(b' ')
            .from_path(self.config.paths.node_path.as_path())
            .unwrap();

        for result in rdr.records() {
            let record = result.unwrap();
            let id = record
                .get(0)
                .expect("Failed to get index 0")
                .parse::<NodeId>()
                .expect("Failed to parse Node ID");
            let lng = record
                .get(1)
                .expect("Failed to get index 1")
                .parse::<f32>()
                .expect("Failed to parse lng");
            let lat = record
                .get(2)
                .expect("Failed to get index 2")
                .parse::<f32>()
                .expect("Failed to parse lat");
            vec.push(Arc::new(DataNode { id, lng, lat }));
        }
        vec.sort_by(|a, b| a.id.partial_cmp(&b.id).unwrap());
        vec
    }

    pub fn read_edge_csv(&self, nodes: &Vec<Arc<DataNode>>) -> Vec<Arc<DataEdge>> {
        let mut vec = Vec::new();

        let mut rdr = ReaderBuilder::new()
            .delimiter(b' ')
            .from_path(self.config.paths.edge_path.as_path())
            .unwrap();

        for result in rdr.records() {
            let record = result.unwrap();
            let id = record
                .get(0)
                .expect("Failed to get index 0")
                .parse::<EdgeId>()
                .expect("Failed to parse Edge ID");
            let ni_id = record
                .get(1)
                .expect("Failed to get index 1")
                .parse::<NodeId>()
                .expect("Failed to parse node i id");
            let nj_id = record
                .get(2)
                .expect("Failed to get index 2")
                .parse::<NodeId>()
                .expect("Failed to parse node j id");

            let index_ni = nodes.binary_search_by(|n| n.id.cmp(&ni_id)).unwrap();
            let index_nj = nodes.binary_search_by(|n| n.id.cmp(&nj_id)).unwrap();
            let ni = nodes.get(index_ni).unwrap();
            let nj = nodes.get(index_nj).unwrap();

            vec.push(Arc::new(DataEdge::new(id, ni.clone(), nj.clone())));
        }

        vec.sort_by(|a, b| a.id.partial_cmp(&b.id).unwrap());
        vec
    }

    pub fn read_query_csv(&self) -> Vec<Arc<Query>> {
        let mut vec = Vec::new();

        let mut rdr = ReaderBuilder::new()
            .delimiter(b' ')
            .from_path(self.config.paths.query_path.as_path())
            .unwrap();

        let mut id = 0;
        for result in rdr.records() {
            id += 1;

            let record = result.unwrap();
            let k = record
                .get(0)
                .expect("Failed to get index 0")
                .parse::<K>()
                .expect("Failed to get k value");

            let mut dimensions = Vec::new();
            let mut i = 1;
            while let Some(str) = record.get(i) {
                if str.is_empty() {
                    break;
                }

                let d = str
                    .parse::<DimensionIndex>()
                    .expect("Failed to parse dimension");
                dimensions.push(d);
                i += 1;
            }
            dimensions.sort();
            let query = Query { id, k, dimensions };
            vec.push(Arc::new(query));
        }

        vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_object_csv() {
        let conf: AppConfig = Default::default();
        let conf = Arc::new(conf);
        let reader = Reader::new(conf);
        let objects = reader.read_object_csv();

        let o1: &Arc<DataObject> = objects.get(0).unwrap();
        let o2: &Arc<DataObject> = objects.get(1).unwrap();
        assert_eq!(o1.id, 1);
        assert_eq!(o2.id, 2);
    }

    #[test]
    fn read_node_edge_csv() {
        let conf: AppConfig = Default::default();
        let conf = Arc::new(conf);
        let reader = Reader::new(conf);
        let nodes = reader.read_node_csv();

        let n1 = nodes.get(0).unwrap();
        let n2 = nodes.get(1).unwrap();

        assert_eq!(n1.id, 1);
        assert_eq!(n2.id, 2);

        let edges = reader.read_edge_csv(&nodes);

        let e1 = edges.get(0).unwrap();
        let e2 = edges.get(1).unwrap();

        assert_eq!(e1.id, 1);
        assert_eq!(e2.id, 2);
    }

    #[test]
    fn read_query_csv() {
        let conf: AppConfig = Default::default();
        let conf = Arc::new(conf);
        let reader = Reader::new(conf);
        let queries = reader.read_query_csv();

        let q1 = queries.get(0).unwrap();
        let q2 = queries.get(1).unwrap();

        assert_eq!(q1.k, 3);
        assert_eq!(q2.k, 4);
    }
}

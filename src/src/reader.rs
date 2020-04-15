use csv::ReaderBuilder;
use std::sync::Arc;

use crate::prelude::*;

pub struct Reader {
    config: AppConfig,
}

impl Reader {
    /// Read object from CSV file
    #[allow(dead_code)]
    pub fn read_object_csv(self) -> Vec<Arc<DataObject>> {
        let mut vec = Vec::new();

        let mut rdr = ReaderBuilder::new()
            .delimiter(b' ')
            .from_path(self.config.paths.object_path)
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
}

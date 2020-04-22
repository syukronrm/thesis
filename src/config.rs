use std::path::{Path, PathBuf};

use crate::prelude::*;

/// Global configuration
#[derive(Clone, Debug)]
pub struct AppConfig {
    pub max_dim: DimensionIndex,
    pub max_dist: f32,
    pub dataset_dir: PathBuf,
    pub paths: Paths,
}

impl Default for AppConfig {
    /// Default data for test
    fn default() -> Self {
        let project_path = Path::new(env!("CARGO_MANIFEST_DIR"));
        let dataset_dir = project_path.join("dataset/test01");

        AppConfig {
            max_dim: 4,
            max_dist: 100.0,
            dataset_dir: dataset_dir.to_path_buf(),
            paths: Paths::new(dataset_dir),
        }
    }
}

/// Paths for all files for dataset
#[derive(Clone, Debug)]
pub struct Paths {
    pub object_path: PathBuf,
    pub node_path: PathBuf,
    pub edge_path: PathBuf,
    pub query_path: PathBuf,
}

impl Paths {
    pub fn new(dataset_dir: PathBuf) -> Self {
        Paths {
            object_path: dataset_dir.join("object.txt"),
            node_path: dataset_dir.join("node.txt"),
            edge_path: dataset_dir.join("edge.txt"),
            query_path: dataset_dir.join("query.txt"),
        }
    }
}

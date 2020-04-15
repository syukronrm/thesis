use std::path::{Path, PathBuf};

/// Global configuration
#[derive(Debug)]
pub struct AppConfig {
    max_dist: f32,
    dataset_dir: PathBuf,
}

impl Default for AppConfig {
    /// Default data for test
    fn default() -> Self {
        let project_path = Path::new(env!("CARGO_MANIFEST_DIR"));
        let dataset_dir = project_path.join("dataset/test01");

        AppConfig {
            max_dist: 100.0,
            dataset_dir,
        }
    }
}
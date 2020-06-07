pub type DimensionIndex = u8;
pub type ObjectId = u32;
pub type EdgeId = u32;
pub type NodeId = u32;
pub type K = u8;
pub type QueryId = u32;
pub type CentroidId = NodeId;

pub use crate::ik::al::bfs_mh::{BfsMinHeap, TraverseState};
pub use crate::ik::al::vor::{DomTraverse, Range, Voronoi};
pub use crate::ik::st::edge::Edge;
pub use crate::ik::st::node::Node;
pub use crate::ik::st::result::ResultVoronoi;

use petgraph::stable_graph::StableGraph as PetGraph;
use petgraph::Undirected;

pub mod edge;
pub mod graph;
pub mod node;
pub mod voronoi;
pub mod query;

pub use edge::*;
pub use graph::Graph;
pub use node::Node;
pub use voronoi::state::State;
pub use query::{Multiqueries, Pair};

pub type PetgraphNodeEdge = PetGraph<Node, Edge, Undirected>;

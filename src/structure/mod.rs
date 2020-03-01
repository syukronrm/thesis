use petgraph::Undirected;
use petgraph::stable_graph::StableGraph as PetGraph;

pub mod edge;
pub mod graph;
pub mod node;
pub mod voronoi;

pub use edge::*;
pub use graph::Graph;
pub use node::Node;
pub use voronoi::state::State;

pub type PetgraphNodeEdge = PetGraph<Node, Edge, Undirected>;

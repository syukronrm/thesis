use petgraph::{Graph as PetGraph, Undirected};

pub mod edge;
pub mod graph;
pub mod node;
pub mod voronoi;

pub use edge::Edge;
pub use graph::Graph;
pub use node::Node;

pub type PetgraphNodeEdge = PetGraph<Node, Edge, Undirected>;

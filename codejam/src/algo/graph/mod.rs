//allow Graph to be imported directly
pub use self::directed_graph::DiGraph;
pub use self::edge_graph::Graph;
pub mod bfs;
pub mod cycles;
mod dfs;
mod directed_graph;
mod edge_graph;
pub mod flow;
pub mod flow2;
pub mod scc;

pub mod connectivity;
pub mod disjointset;

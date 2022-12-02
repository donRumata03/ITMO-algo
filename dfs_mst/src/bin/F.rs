//! Building condensation of the directed graph from input
//! and counting the number of edges in condensation (excluding self-loops and parallel edges)
//! Use the Kosaraju algorithm

extern crate dfs_mst;

use self::dfs_mst::{find_edge_biconnected_components, print_vec};
use self::dfs_mst::{Graph, DFSSpace, InputReader, Edge, VisitColor};


fn main() {
	let mut input = InputReader::new();
	// let mut output = OutputWriter::new();

	let graph = Graph::from_stdin(&mut input, true);
	let n = graph.vertexes();

	let mut dfs_space = DFSSpace::new(&graph);
	let (condensation_graph, vertex_decomposition) = dfs_space.condensation(&graph);

	// Print the number of edges in condensation
	println!("{}", condensation_graph.edges());
}

/*
4 4
2 1
3 2
2 3
4 3
----------------
2>1
3>2
2>3
4>3
----------------
 */

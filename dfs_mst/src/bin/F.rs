//! Building condensation of the directed graph from input
//! and counting the number of edges in condensation (excluding self-loops and parallel edges)
//! Use the Kosaraju algorithm

extern crate dfs_mst;

use petgraph::algo::condensation;
use self::dfs_mst::{find_edge_biconnected_components, print_vec};
use self::dfs_mst::{Graph, DFSSpace, InputReader, Edge, VisitColor};


fn main() {
	let mut input = InputReader::new();
	// let mut output = OutputWriter::new();

	let graph = Graph::from_stdin(&mut input, false);
	let n = graph.vertexes();

	let condensation = condensation(graph);

	let mut components_of_vertex = vec![0; n];
	for (i, component) in edge_biconnected_components.iter().enumerate() {
		for v in component {
			components_of_vertex[*v] = i;
		}
	}

	// Print components of vertexes
	println!("{}", edge_biconnected_components.len());
	print_vec(components_of_vertex.iter().map(|&x| (x + 1)).collect());
}

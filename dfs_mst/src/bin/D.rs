// Decomposing undirected graph into biconnected components

extern crate dfs_mst;

use self::dfs_mst::{find_edge_biconnected_components, print_vec};
use self::dfs_mst::{Graph, DFSSpace, InputReader, Edge, VisitColor};


fn main() {
	let mut input = InputReader::new();
	// let mut output = OutputWriter::new();

	let graph = Graph::from_stdin(&mut input, false);
	let n = graph.vertexes();

	let edge_biconnected_components = find_edge_biconnected_components(graph);

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

/*
6 7
1 2
2 3
2 4
2 5
4 5
1 3
3 6
----------------
1-2
2-3
2-4
2-5
4-5
1-3
3-6

 */

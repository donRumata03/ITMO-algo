// Searching cutting points in undirected graph

extern crate dfs_mst;

use self::dfs_mst::{Graph, DFSSpace, InputReader, Edge, VisitColor};


fn main() {
	let mut input = InputReader::new();
	// let mut output = OutputWriter::new();

	let graph = Graph::from_stdin(&mut input, false);

	let mut dfs_space = DFSSpace::new(&graph);
	let condensation = dfs_space.condensation(&graph);
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

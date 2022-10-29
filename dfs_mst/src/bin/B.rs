// Searching bridges in undirected graph

extern crate dfs_mst;

use self::dfs_mst::{Graph, DFSSpace, InputReader, Edge, VisitColor};


fn main() {
	let mut input = InputReader::new();
	// let mut output = OutputWriter::new();

	let graph = Graph::from_stdin(&mut input, false);

	// Find bridges
	// Bridge is an edge that if removed, the graph will be disconnected
	// In dfs, if in the subtree of a node, there are edges that point to a node that is higher in the dfs tree,
	// then the edge is not a bridge

	let mut bridges = DFSSpace::new(&graph).find_bridges(&graph);

	// Print all bridges
	bridges.sort();
	println!("{}", bridges.len());
	for bridge in bridges {
		println!("{}", bridge + 1);
	}
}

/*

6 7
1 2
2 3
3 4
1 3
4 5
4 6
5 6

——————————————

3 3
1 2
2 3
3 1

——————————————

4 4
1 2
2 3
3 1
4 1
——————————————

3 2
1 2
2 3

 */

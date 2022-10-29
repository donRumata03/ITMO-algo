// Searching cutting points in undirected graph

extern crate dfs_mst;

use self::dfs_mst::{Graph, DFSSpace, InputReader, Edge, VisitColor};


fn main() {
	let mut input = InputReader::new();
	// let mut output = OutputWriter::new();

	let graph = Graph::from_stdin(&mut input, false);

	// Find cutting points
	// Cutting point is a node that if removed, the graph will be disconnected
	// In dfs, if a vertex v (except for the root) has a child u for which highest_reachable[u] >= t_in[v],
	// then v is a cutting point
	// For root r: r is a cutting point <=> r has at least 2 children

	let mut dfs_space = DFSSpace::new(&graph);
	let mut cutting_points = dfs_space.find_cutting_points(&graph);

	// Print all cutting points in ascending order on a single line
	cutting_points.sort();
	println!("{}\n{}", cutting_points.len(), cutting_points.iter().map(|&x| (x + 1).to_string()).collect::<Vec<String>>().join(" "));
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

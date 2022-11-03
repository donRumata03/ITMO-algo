extern crate dfs_mst;

use dfs_mst::{
	Graph,
	DFSSpace,
	InputReader,
	kruskal,
	WeightedGraph,
	WeightedEdge
};


fn main() {
	let mut input = InputReader::new();
	// let mut output = OutputWriter::new();

	let graph = WeightedGraph::<i64>::weighted_from_stdin(&mut input, false);
	let mst = kruskal(&graph);
	println!("{}", mst
		.iter()
		.map(|&WeightedEdge::<i64>{ to, edge_index, weight }| weight)
		.sum::<i64>()
	);
}

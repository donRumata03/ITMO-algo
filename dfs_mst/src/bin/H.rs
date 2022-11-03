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

	let n = input.next();
	// Input integer weight matrix n×n.
	let mut matrix = Vec::new();
	for _ in 0..n {
		matrix.push(Vec::new());
		for _ in 0..n {
			let weight = input.next::<i64>();
			matrix.last_mut().unwrap().push(weight);
		}
	}

	let graph = WeightedGraph::<i64>::from_weight_matrix(&matrix);
	let mst = kruskal(&graph);
	println!("{}", mst
		.iter()
		.map(|&WeightedEdge::<i64>{ to, edge_index, weight }| weight)
		.sum()
		.unwrap_or_default()
	);
}

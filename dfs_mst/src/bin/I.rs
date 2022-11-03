extern crate dfs_mst;

use dfs_mst::{
	Graph, DFSSpace, InputReader, kruskal, WeightedGraph, WeightedEdge, kruskal_edges
};

// f64 wrapper for WeightedGraph.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Default)]
pub struct F64(f64);

impl F64 {
	pub fn new(value: f64) -> Self {
		Self(value)
	}
}

impl Ord for F64 {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.partial_cmp(other).unwrap()
	}
}

impl Eq for F64 {}

fn main() {
	let mut input = InputReader::new();
	// let mut output = OutputWriter::new();

	let n = input.next();
	// Input integer weight matrix n×n.
	let points = (0..n).map(|_| (input.next::<i64>(), input.next::<i64>())).collect::<Vec<_>>();

	// Compute dist matrix.
	let mut dist_matrix = vec![vec![F64(0.); n]; n];
	for i in 0..n {
		for j in 0..n {
			// Euclidean distance.
			dist_matrix[i][j] = F64((((points[i].0 - points[j].0).pow(2) + (points[i].1 - points[j].1).pow(2)) as f64).sqrt());
		}
	}

	let mut edges = Vec::new();
	for i in 0..n {
		for j in 0..n {
			edges.push((i, WeightedEdge::<F64> {
				to: j,
				edge_index: j,
				weight: dist_matrix[i][j]
			}));
		}
	}

	let mst = kruskal_edges(&edges, n);
	println!("{}", mst
		.iter()
		.map(|&WeightedEdge::<F64>{ to, edge_index, weight }| weight.0)
		.sum::<f64>()
	);
}

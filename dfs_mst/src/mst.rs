use crate::{DSU, Graph, WeightedEdge, WeightedGraph};

/// Returns the list of edges in the MST (their indices).
pub fn kruskal<T: Copy + Ord>(graph: &WeightedGraph<T>) -> Vec<WeightedEdge<T>> {
	let n = graph.vertexes();
	let mut edges = vec![];
	for i in 0..n {
		for &edge in &graph.edges[i] {
			edges.push((i, edge));
		}
	}
	edges.sort_by_key(|&(_, WeightedEdge { to, edge_index, weight: weight })| weight);
	let mut dsu = DSU::new(n);
	let mut mst = vec![];
	for (from, WeightedEdge { to, edge_index, weight }) in edges {
		if dsu.find(from) != dsu.find(to) {
			dsu.union(from, to);
			mst.push(WeightedEdge { to, edge_index, weight });
		}
	}

	mst
}

/// Returns the list of edges in the MST (their indices).
pub fn kruskal_edges<T: Copy + Ord>(graph: &Vec<(usize, WeightedEdge<T>)>, n: usize) -> Vec<WeightedEdge<T>> {
	let mut edges = graph.clone();

	edges.sort_by_key(|&(_, WeightedEdge { to, edge_index, weight: weight })| weight);
	let mut dsu = DSU::new(n);
	let mut mst = vec![];
	for (from, WeightedEdge { to, edge_index, weight }) in edges {
		if dsu.find(from) != dsu.find(to) {
			dsu.union(from, to);
			mst.push(WeightedEdge { to, edge_index, weight });
		}
	}

	mst
}

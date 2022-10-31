use crate::{DSU, Graph};

pub fn kruskal(graph: &Graph) -> (usize, Vec<(usize, usize)>) {
	let mut edges = vec![];
	for i in 0..n {
		for j in 0..n {
			if i != j && g[i][j] != 0 {
				edges.push((g[i][j], i, j));
			}
		}
	}
	edges.sort();
	let mut dsu = DSU::new(n);
	let mut mst = vec![];
	for (w, u, v) in edges {
		if dsu.find(u) != dsu.find(v) {
			dsu.union(u, v);
			mst.push((w, u, v));
		}
	}
	mst
}
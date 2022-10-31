use crate::*;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VisitColor {
	White,
	Gray,
	Black,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Edge {
	pub to: usize,
	pub edge_index: usize,
}

// Graph as an adjacency list
#[derive(Debug, Clone)]
pub struct Graph {
	pub edges: Vec<Vec<Edge>>,
	pub total_edges: usize,
}

impl Graph {
	pub fn new(n: usize) -> Self {
		Graph {
			edges: vec![Vec::new(); n],
			total_edges: 0,
		}
	}

	pub fn from_stdin(input_reader: &mut InputReader<Stdin>, directed: bool) -> Self {
		let n = input_reader.next::<usize>();
		let m = input_reader.next::<usize>();
		let mut graph = Graph::new(n);
		for _ in 0..m {
			let from = input_reader.next::<usize>();
			let to = input_reader.next::<usize>();
			if directed {
				graph.add_directed_edge(from - 1, to - 1);
			} else {
				graph.add_undirected_edge(from - 1, to - 1);
			}
		}
		graph
	}

	pub fn add_indexed_directed_edge(&mut self, from: usize, to: usize, edge_index: usize) {
		self.edges[from].push(Edge { to, edge_index });
	}

	pub fn add_undirected_edge(&mut self, from: usize, to: usize) {
		let edge_index = self.total_edges;
		self.add_indexed_directed_edge(from, to, edge_index);
		self.add_indexed_directed_edge(to, from, edge_index);
		self.total_edges += 1;
	}

	pub fn add_directed_edge(&mut self, from: usize, to: usize) {
		let edge_index = self.total_edges;
		self.add_indexed_directed_edge(from, to, edge_index);
		self.total_edges += 1;
	}

	pub fn vertexes(&self) -> usize {
		self.edges.len()
	}

	pub fn edges(&self) -> usize {
		self.total_edges
	}

	pub fn remove_edges(&mut self, edges: &[usize]) {
		let mut removed_edges = vec![false; self.total_edges];
		for edge in edges {
			removed_edges[*edge] = true;
		}
		for edges in &mut self.edges {
			edges.retain(|edge| !removed_edges[edge.edge_index]);
		}
		self.total_edges -= edges.len();
	}

	/// Vertexes have the same indexes as in the original graph
	/// Edges have the same indexes as in the original graph
	/// But `from` and `to` of edges are swapped
	pub fn reversed(&self) -> Graph {
		let mut reversed = Graph::new(self.vertexes());
		for (from, edges) in self.edges.iter().enumerate() {
			for edge in edges {
				reversed.add_indexed_directed_edge(edge.to, from, edge.edge_index);
				reversed.total_edges += 1;
			}
		}
		reversed
	}

	/// Deduplicate edges
	/// Edges are also renumerated (such that if `consider_inverse_edges_equal` is true,
	/// straight and inverse edges will have the same index in the new graph)
	pub fn deduplicated(&self, consider_inverse_edges_equal: bool) -> Self {
		// Group edges by sorted pair of `end` and `to`
		let double_ended_edges = self.edges.iter()
			.enumerate()
			.map(|(i, edges)| edges.iter().map(|edge| (i, edge.to, edge.edge_index)).collect::<Vec<(usize, usize, usize)>>())
			.flatten()
			.collect();

		let mut deduplicated = Graph::new(self.vertexes());

		for (new_index, ((from, to), _group)) in
			group_by(&double_ended_edges,
			         |&(from, to, index)| if consider_inverse_edges_equal { minmax(from, to) } else { (from, to) })
				.into_iter()
				.enumerate()
		{
			// Straight edge
			deduplicated.add_indexed_directed_edge(from, to, new_index);
			deduplicated.total_edges += 1;
			// Inverse edge
			if consider_inverse_edges_equal {
				deduplicated.add_indexed_directed_edge(to, from, new_index);
				deduplicated.total_edges += 1;
			}
		}

		deduplicated
	}
}


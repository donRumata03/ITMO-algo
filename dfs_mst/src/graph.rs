use crate::*;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VisitColor {
	White,
	Gray,
	Black,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Edge {
	to: usize,
	edge_index: usize,
}

// Graph as an adjacency list
#[derive(Debug, Clone)]
pub struct Graph {
	edges: Vec<Vec<Edge>>,
	total_edges: usize,
}

impl Graph {
	pub fn new(n: usize) -> Self {
		Graph {
			edges: vec![Vec::new(); n],
			total_edges: 0,
		}
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
}

#[derive(Debug, Clone)]
pub struct DFSSpace {
	pub time: usize,
	pub visit_colors: Vec<VisitColor>,
	pub t_in: Vec<usize>,
	pub t_out: Vec<usize>,
}

impl DFSSpace {
	pub fn new(graph: &Graph) -> Self {
		let n = graph.vertexes();
		DFSSpace {
			time: 0,
			visit_colors: vec![VisitColor::White; n],
			t_in: vec![0; n],
			t_out: vec![0; n],
		}
	}

	pub fn topological_sort(&mut self, graph: &Graph) -> Option<Vec<usize>> {
		let mut order = Vec::new();
		for v in 0..graph.vertexes() {
			if self.visit_colors[v] == VisitColor::White {
				self.dfs(graph, v, &mut order);
			}
		}
		order.reverse();
		if order.len() == graph.vertexes() {
			Some(order)
		} else {
			None
		}
	}

	fn dfs(&mut self, graph: &Graph, v: usize, order: &mut Vec<usize>) {
		self.visit_colors[v] = VisitColor::Gray;
		self.t_in[v] = self.time;
		self.time += 1;
		for edge in &graph.edges[v] {
			let to = edge.to;
			if self.visit_colors[to] == VisitColor::White {
				self.dfs(graph, to, order);
			} else if self.visit_colors[to] == VisitColor::Gray {
				// Cycle detected
				return;
			}
		}
		self.visit_colors[v] = VisitColor::Black;
		self.t_out[v] = self.time;
		self.time += 1;
		order.push(v);
	}
}

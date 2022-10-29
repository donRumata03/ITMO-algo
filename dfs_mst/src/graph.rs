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
}

#[derive(Debug, Clone)]
pub struct DFSSpace {
	pub time: usize,
	pub visit_colors: Vec<VisitColor>,
	pub t_in: Vec<usize>,
	pub t_out: Vec<usize>,
	pub children: Vec<Vec<usize>>,
}

impl DFSSpace {
	pub fn new(graph: &Graph) -> Self {
		let n = graph.vertexes();
		DFSSpace {
			time: 0,
			visit_colors: vec![VisitColor::White; n],
			t_in: vec![0; n],
			t_out: vec![0; n],
			children: vec![Vec::new(); n],
		}
	}

	pub fn clear(&mut self) {
		self.time = 0;
		self.visit_colors.fill(VisitColor::White);
		self.t_in.fill(0);
		self.t_out.fill(0);
		self.children.fill(Vec::new());
	}

	pub fn ignore_vertexes(&mut self, vertexes: &[usize]) {
		for &vertex in vertexes {
			self.visit_colors[vertex] = VisitColor::Black;
		}
	}

	pub fn topological_sort(&mut self, graph: &Graph) -> Option<Vec<usize>> {
		let mut order = Vec::new();
		for v in 0..graph.vertexes() {
			if self.visit_colors[v] == VisitColor::White {
				self.topsort_dfs(graph, v, &mut order);
			}
		}
		order.reverse();
		if order.len() == graph.vertexes() {
			Some(order)
		} else {
			None
		}
	}

	fn topsort_dfs(&mut self, graph: &Graph, v: usize, order: &mut Vec<usize>) {
		self.visit_colors[v] = VisitColor::Gray;
		self.t_in[v] = self.time;
		self.time += 1;
		for edge in &graph.edges[v] {
			let to = edge.to;
			if self.visit_colors[to] == VisitColor::White {
				self.topsort_dfs(graph, to, order);
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

	pub fn test_acyclic(&mut self, graph: &Graph) -> bool {
		self.topological_sort(graph).is_some()
	}

	pub fn find_bridges(&mut self, graph: &Graph) -> Vec<usize> { // List of edge indexes of bridges
		let mut bridges = Vec::new();
		let mut highest_reachable = vec![0; graph.vertexes()];
		for v in 0..graph.vertexes() {
			if self.visit_colors[v] == VisitColor::White {
				self.bridge_dfs(v, graph, &mut bridges, &mut highest_reachable, None);
			}
		}
		bridges
	}

	fn bridge_dfs(&mut self, node: usize, graph: &Graph, bridges: &mut Vec<usize>, highest_reachable: &mut Vec<usize>, edge_to_parent: Option<Edge>) {
		self.visit_colors[node] = VisitColor::Gray;
		self.t_in[node] = self.time;
		highest_reachable[node] = self.time;
		self.time += 1;

		for edge in &graph.edges[node] {
			let to = edge.to;
			if self.visit_colors[to] == VisitColor::White {
				self.bridge_dfs(to, graph, bridges, highest_reachable, Some(Edge { to: node, edge_index: edge.edge_index }));
				highest_reachable[node] = min(highest_reachable[node], highest_reachable[to]);
			} else if self.visit_colors[to] == VisitColor::Gray {
				// upper edge from node itself (handle parent separately)
				match edge_to_parent {
					Some(parent_edge) => {
						if parent_edge.to != to {
							highest_reachable[node] = min(highest_reachable[node], self.t_in[to]);
						}
					},
					None => highest_reachable[node] = min(highest_reachable[node], self.t_in[to]),
				}
			}
		}

		// If the node is not the root of the dfs tree and in the subtree there is an edge to a node that is higher in the dfs tree,
		// then the edge is not a bridge
		// Root of the dfs tree doesn't have any edges «associated» with it
		match edge_to_parent {
			Some(Edge { to: _parent, edge_index }) if highest_reachable[node] == self.t_in[node] => {
				bridges.push(edge_index);
			}, _ => {}
		}

		self.visit_colors[node] = VisitColor::Black;
	}

	pub fn find_cutting_points(&mut self, graph: &Graph) -> Vec<usize> { // List of indexes of vertexes that are cutting points
		let mut cutting_points = Vec::new();
		let mut highest_reachable = vec![0; graph.vertexes()];
		for v in 0..graph.vertexes() {
			if self.visit_colors[v] == VisitColor::White {
				self.cutting_point_dfs(v, graph, &mut cutting_points, &mut highest_reachable, None);
			}
		}
		cutting_points.sort();
		cutting_points.dedup();

		// dbg!(self);
		// dbg!(highest_reachable);


		cutting_points
	}

	fn cutting_point_dfs(&mut self, node: usize, graph: &Graph, cutting_points: &mut Vec<usize>, highest_reachable: &mut Vec<usize>, edge_to_parent: Option<Edge>) {
		self.visit_colors[node] = VisitColor::Gray;
		self.t_in[node] = self.time;
		highest_reachable[node] = self.time;
		self.time += 1;

		for edge in &graph.edges[node] {
			let to = edge.to;
			// Continue if to is a parent
			if edge_to_parent.is_some() && edge_to_parent.unwrap().to == to {
				continue;
			}
			if self.visit_colors[to] == VisitColor::White {
				self.cutting_point_dfs(to, graph, cutting_points, highest_reachable, Some(Edge { to: node, edge_index: edge.edge_index }));
				highest_reachable[node] = min(highest_reachable[node], highest_reachable[to]);
				if edge_to_parent.is_some() && highest_reachable[to] >= self.t_in[node] {
					cutting_points.push(node);
				}
				self.children[node].push(to);
			} else if self.visit_colors[to] == VisitColor::Gray {
				// upper edge from node itself
				highest_reachable[node] = min(highest_reachable[node], self.t_in[to]);
			}
		}

		if edge_to_parent.is_none() {
			if self.children[node].len() > 1 {
				cutting_points.push(node);
			}
		}

		self.visit_colors[node] = VisitColor::Black;
	}
}

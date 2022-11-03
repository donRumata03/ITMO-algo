use std::cmp::min;
use crate::{Decomposition, Edge, Graph, VisitColor};

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

	pub fn dfs_preorder_with<F>(&mut self, graph: &Graph, v: usize, run_for_vertex: &mut F)
		where F: FnMut(usize)
	{
		self.visit_colors[v] = VisitColor::Gray;
		run_for_vertex(v);

		for edge in &graph.edges[v] {
			let to = edge.to;
			if self.visit_colors[to] == VisitColor::White {
				self.dfs_preorder_with(graph, to, run_for_vertex);
			}
		}
		self.visit_colors[v] = VisitColor::Black;
	}


	pub fn find_connected_components(&mut self, graph: &Graph) -> Decomposition {
		let mut components = Vec::new();
		for vertex in 0..graph.vertexes() {
			if self.visit_colors[vertex] == VisitColor::White {
				components.push(Vec::new());
				self.dfs_preorder_with(
					graph, vertex, &mut |vertex| components.last_mut().unwrap().push(vertex)
				);
			}
		}
		Decomposition::from_component_list(components)
	}


	pub fn topological_sort(&mut self, graph: &Graph) -> (Vec<usize>, bool) {
		let mut order = Vec::new();
		let mut has_cycle = false;
		for v in 0..graph.vertexes() {
			if self.visit_colors[v] == VisitColor::White {
				has_cycle |= self.topsort_dfs(graph, v, &mut order);
			}
		}
		order.reverse();
		(order, !has_cycle)
	}

	fn topsort_dfs(&mut self, graph: &Graph, v: usize, order: &mut Vec<usize>) -> bool {
		let mut has_cycle = false;
		self.visit_colors[v] = VisitColor::Gray;
		self.t_in[v] = self.time;
		self.time += 1;
		for edge in &graph.edges[v] {
			let to = edge.to;
			if self.visit_colors[to] == VisitColor::White {
				has_cycle |= self.topsort_dfs(graph, to, order);
			} else if self.visit_colors[to] == VisitColor::Gray {
				// Cycle detected
				has_cycle = true;
			}
		}
		self.visit_colors[v] = VisitColor::Black;
		self.t_out[v] = self.time;
		self.time += 1;
		order.push(v);
		has_cycle
	}

	pub fn test_acyclic(&mut self, graph: &Graph) -> bool {
		self.topological_sort(graph).1
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
				self.bridge_dfs(to, graph, bridges, highest_reachable, Some(Edge::new(node, edge.edge_index)));
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
			Some(Edge {to: _parent, edge_index, weight: _}) if highest_reachable[node] == self.t_in[node] => {
				bridges.push(edge_index);
			}, _ => {}
		}

		self.visit_colors[node] = VisitColor::Black;
	}

	/// Returns both the list of indexes of vertexes that are cutting points
	/// and the partition of the graph into VERTEX-biconnected components
	pub fn find_cutting_points_with_components(&mut self, graph: &Graph) -> (Vec<usize>, Decomposition) {
		let mut cutting_points = Vec::new();
		let mut highest_reachable = vec![0; graph.vertexes()];
		let mut components = Vec::new();
		let mut edge_stack = Vec::new();
		let mut edge_visited = vec![false; graph.edges()];
		// We could add to stack only edges to White and Grey vertexes (except THE vertex to parent)
		// but here we can have parallel edges, so we need to check if the edge is visited
		for v in 0..graph.vertexes() {
			if self.visit_colors[v] == VisitColor::White {
				self.cutting_point_dfs(graph, v, None, &mut highest_reachable, &mut cutting_points, &mut components, &mut edge_stack, &mut edge_visited);
			}
		}
		cutting_points.sort();
		cutting_points.dedup();

		(cutting_points, Decomposition::from_component_list(components))
	}

	fn cutting_point_dfs(&mut self, graph: &Graph,
	                     node: usize, edge_to_parent: Option<Edge>,
	                     highest_reachable: &mut Vec<usize>,
	                     cutting_points: &mut Vec<usize>,
	                     components: &mut Vec<Vec<usize>>,
	                     edge_stack: &mut Vec<Edge>,
	                     edge_visited: &mut Vec<bool>
	)
	{
		self.visit_colors[node] = VisitColor::Gray;
		self.t_in[node] = self.time;
		highest_reachable[node] = self.time;
		self.time += 1;

		for &edge in &graph.edges[node] {
			let to = edge.to;
			// Not just continue if to is a parent (cause we can have parallel edges…)
			// But if this is THE edge from which we came from parent
			if edge_to_parent.is_some() && edge_to_parent.unwrap().edge_index == edge.edge_index {
				continue;
			}
			if !edge_visited[edge.edge_index] {
				edge_visited[edge.edge_index] = true;
				edge_stack.push(edge);
			}
			if self.visit_colors[to] == VisitColor::White {
				self.cutting_point_dfs(graph, to, Some(Edge::new(node, edge.edge_index)),
				                       highest_reachable, cutting_points, components, edge_stack, edge_visited);
				highest_reachable[node] = min(highest_reachable[node], highest_reachable[to]);
				if edge_to_parent.is_some() && highest_reachable[to] >= self.t_in[node] {
					cutting_points.push(node);
				}
				// Add a new component if node is cutting point or root
				if edge_to_parent.is_none() || highest_reachable[to] >= self.t_in[node] {
					let mut component = Vec::new();
					loop {
						let stack_edge = edge_stack.pop().unwrap();
						component.push(stack_edge.edge_index);
						if stack_edge.edge_index == edge.edge_index {
							break;
						}
					}
					components.push(component);
				}
				self.children[node].push(to);
			} else if self.visit_colors[to] == VisitColor::Gray {
				// upper edge from node itself (the parent is already ignored via continue)
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

	// Compresses the components of strong connectivity and returns:
	// — New graph of strong connectivity components (in the topological order)
	// — Decomposition of VERTEXES of the original graph into strong connectivity components
	//   such that i'th component of the decomposition is the set of vertexes
	//   that belong to the i'th component of the new graph
	pub fn condensation(&mut self, graph: &Graph) -> (Graph, Decomposition) {
		let quasi_topsort = self.topological_sort(graph).0;
		self.clear();
		let reversed_graph = graph.reversed();
		let mut components = Vec::new();
		for &root in &quasi_topsort {
			if self.visit_colors[root] == VisitColor::White {
				components.push(Vec::new());
				self.dfs_preorder_with(&reversed_graph, root, &mut |v| {
					components.last_mut().unwrap().push(v);
				});
			}
		}

		let mut decomposition = Decomposition::from_component_list(components);

		let mut condensation_graph = Graph::new(decomposition.component_list.len());
		for v in 0..graph.vertexes() {
			for &edge in &graph.edges[v] {
				let from = decomposition.component_of(v);
				let to = decomposition.component_of(edge.to);
				if from != to {
					condensation_graph.add_directed_edge(from, to);
				}
			}
		}

		(condensation_graph.deduplicated(false), decomposition)
	}
}

pub fn find_edge_biconnected_components(mut graph: Graph) -> Decomposition {
	let mut dfs_space = DFSSpace::new(&graph);
	let bridges = dfs_space.find_bridges(&mut graph);

	graph.remove_edges(&bridges);

	DFSSpace::new(&graph).find_connected_components(&graph)
}

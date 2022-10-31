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

	/// Vertexes have the same indexes as in the original graph
	/// Edges have the same indexes as in the original graph
	/// But `from` and `to` of edges are swapped
	pub fn reversed(&self) -> Self {
		let mut reversed = Graph::new(self.vertexes());
		for (from, edges) in self.edges.iter().enumerate() {
			for edge in edges {
				reversed.add_indexed_directed_edge(edge.to, from, edge.edge_index);
				reversed.total_edges += 1;
			}
		}
		reversed
	}
}

pub struct Decomposition {
	pub elements: usize,
	pub component_list: Vec<Vec<usize>>,
	pub component_map: Vec<usize>,
}

impl Decomposition {
	pub fn from_component_list(component_list: Vec<Vec<usize>>) -> Self {
		let elements = component_list.iter().map(|x| x.len()).sum();

		debug_assert_eq!(elements, component_list.iter().flatten().max().map(|x| x + 1).unwrap_or_default(),
		                 "Components should be indexed from 0 to n - 1 without gaps");

		debug_assert_eq!(elements, HashSet::<usize>::from_iter(
			component_list.iter()
				.flatten()
				.cloned()
		).len(), "There are duplicate vertexes in component list");

		let mut component_map = vec![0; elements];
		for (i, component) in component_list.iter().enumerate() {
			for v in component {
				component_map[*v] = i;
			}
		}
		Decomposition {
			elements,
			component_list,
			component_map,
		}
	}

	pub fn from_component_map(component_map: Vec<usize>) -> Self {
		let elements = component_map.len();
		let component_number = component_map.iter().max().map(|&n| n + 1).unwrap_or_default();

		let mut component_list = vec![Vec::new(); component_number];
		for (i, component) in component_map.iter().enumerate() {
			component_list[*component].push(i);
		}

		// Debug assert that all indexes from 0 to n - 1 are present using array
		debug_assert!(component_list.iter().all(|x| !x.is_empty()),
			"All indexes from 0 to n - 1 should be present in component map");

		Decomposition {
			elements,
			component_list,
			component_map,
		}
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
				self.cutting_point_dfs(graph, to, Some(Edge { to: node, edge_index: edge.edge_index }),
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
	// — New graph of strong connectivity components
	// — For each vertex of new graph: List of vertexes belonging to the corresponding component
	pub fn condensation(&mut self, graph: &Graph) -> (Graph, Decomposition) {
		let quasi_topsort = self.topological_sort(graph).0;
		let reversed_graph = graph.reversed();
		let mut component_vertexes = vec![0; graph.vertexes()];
		unimplemented!()
	}
}

pub fn find_edge_biconnected_components(mut graph: Graph) -> Decomposition {
	let mut dfs_space = DFSSpace::new(&graph);
	let bridges = dfs_space.find_bridges(&mut graph);

	graph.remove_edges(&bridges);

	DFSSpace::new(&graph).find_connected_components(&graph)
}

// Searching bridges in undirected graph

extern crate dfs_mst;

use dfs_mst::{
	Graph,
	DFSSpace,
	InputReader
};


fn main() {
	let mut graph = Vec::new();

	let mut input = InputReader::new();
	// let mut output = OutputWriter::new();

	let n: usize = input.next();
	let m: usize = input.next();

	// Add nodes with id 0..n
	for _ in 0..n {
		graph.push(Vec::new());
	}

	// Read m edges and add them to the graph
	for edge_index in 0..m {
		let u: usize = input.next();
		let v: usize = input.next();
		graph[u - 1].push(Edge { to: v - 1, edge_index });
		graph[v - 1].push(Edge { to: u - 1, edge_index });
	}

	// Find bridges
	// Bridge is an edge that if removed, the graph will be disconnected
	// In dfs, if in the subtree of a node, there are edges that point to a node that is higher in the dfs tree,
	// then the edge is not a bridge

	let mut bridges = Vec::new(); // Indexes of edges that are bridges
	let mut t_in = vec![0; n]; // First visit order of nodes
	let mut visit_color = vec![VisitColor::White; n]; // Traversal color of nodes
	let mut time = 0; // Counter for visit order
	let mut highest_reachable = vec![usize::MAX; n]; // Smallest t_in of node to which there is an edge from the subtree of the node

	for node in 0..n {
		if visit_color[node] == VisitColor::White {
			dfs(node, &graph, &mut t_in, &mut visit_color, &mut time, &mut bridges, &mut highest_reachable, None);
		}
	}

	// Debug all
	// dbg!(bridges);
	// dbg!(t_in);
	// dbg!(visit_color);
	// dbg!(highest_reachable);

	// Print all bridges
	bridges.sort();
	println!("{}", bridges.len());
	for bridge in bridges {
		println!("{}", bridge + 1);
	}
}

fn dfs(node: usize, graph: &Vec<Vec<Edge>>, t_in: &mut Vec<usize>, visit_color: &mut Vec<VisitColor>, time: &mut usize, bridges: &mut Vec<usize>, highest_reachable: &mut Vec<usize>, parent: Option<usize>) {
	visit_color[node] = VisitColor::Gray;
	t_in[node] = *time;
	highest_reachable[node] = *time;
	*time += 1;


	for edge in &graph[node] {
		if visit_color[edge.to] == VisitColor::White {
			dfs(edge.to, graph, t_in, visit_color, time, bridges, highest_reachable, Some(node));
			highest_reachable[node] = min(highest_reachable[node], highest_reachable[edge.to]);
		} else if visit_color[edge.to] == VisitColor::Gray {
			// Handle parent separately
			// upper edge from node itself
			if parent != Some(edge.to) {
				highest_reachable[node] = min(highest_reachable[node], t_in[edge.to]);
			}
		}
	}

	// If the node is not the root of the dfs tree and in the subtree there is an edge to a node that is higher in the dfs tree,
	// then the edge is not a bridge
	match parent {
		Some(parent) if highest_reachable[node] == t_in[node] => { // Get index of the edge from the parent to the node
			let edge_index = graph[parent.unwrap()].iter().filter(|&edge| edge.to == node).nth(0);
			if edge_index.is_none() { return; }
			bridges.push(edge_index.unwrap().edge_index);
		}, _ => {}
	}

	visit_color[node] = VisitColor::Black;
}

/*

6 7
1 2
2 3
3 4
1 3
4 5
4 6
5 6

——————————————

3 3
1 2
2 3
3 1

——————————————

4 4
1 2
2 3
3 1
4 1
——————————————

3 2
1 2
2 3

 */

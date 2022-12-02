use std::collections::HashMap;
use std::hash::Hash;
use crate::{DFSSpace, Graph};


pub fn solve_not_numbered_2sat<T: Ord + Clone + Hash>(clauses: &[(T, bool, T, bool)]) -> Option<(Vec<bool>, HashMap<T, usize>)> {
	// Use hash map to convert T to usize
	let mut map = HashMap::<T, usize>::new();
	let mut counter = 0;
	for (a, _, b, _) in clauses {
		if !map.contains_key(a) {
			map.insert(a.clone(), counter);
			counter += 1;
		}
		if !map.contains_key(b) {
			map.insert(b.clone(), counter);
			counter += 1;
		}
	}

	// Use the enumeration to build &[(usize, bool, usize, bool)]
	let mut numbered_clauses = Vec::<(usize, bool, usize, bool)>::new();
	for &(ref a, a_sign, ref b, b_sign) in clauses {
		let a = map[a];
		let b = map[b];
		numbered_clauses.push((a, a_sign, b, b_sign));
	}

	Some((solve_2sat(&numbered_clauses)?, map))
}

pub fn solve_2sat(clauses: &[(usize, bool, usize, bool)]) -> Option<Vec<bool>> {
	// dbg!(clauses);
	let n = clauses.iter().map(|&(a, _, b, _)| a.max(b)).max().unwrap() + 1;
	let mut graph = Graph::new(2 * n);

	for &(a, a_sign, b, b_sign) in clauses {
		let a = a * 2 + if a_sign { 0 } else { 1 };
		let b = b * 2 + if b_sign { 0 } else { 1 };
		graph.add_directed_edge(a ^ 1, b);
		graph.add_directed_edge(b ^ 1, a);
	}

	let full_assignment = two_sat_assign(&graph)?;
	let mut assignment = vec![false; n];
	for i in 0..n {
		assignment[i] = full_assignment[i * 2];
	}
	Some(assignment)
}

fn two_sat_assign(graph: &Graph) -> Option<Vec<bool>> {
	let mut dfs_space = DFSSpace::new(graph);
	let (condensation_graph, components) = dfs_space.condensation(graph);
	// dbg!(&graph);
	// dbg!(&condensation_graph);
	// dbg!(&components);

	let mut values = vec![false; graph.vertexes()];

	for vertex in 0..graph.vertexes() {
		if components.component_of(vertex) == components.component_of(vertex ^ 1) {
			return None;
		}

		values[vertex] = components.component_of(vertex) > components.component_of(vertex ^ 1);
	}

	Some(values)
}
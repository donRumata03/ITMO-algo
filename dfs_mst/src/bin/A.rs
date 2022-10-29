use dfs_mst::*;

fn main() {
	let mut input = InputReader::new();
	// let mut output = OutputWriter::new();

	let n: usize = input.next();
	let m: usize = input.next();

	let mut graph = crate::Graph::new(n);

	// Read m edges and add them to the graph
	for _ in 0..m {
		let u: usize = input.next();
		let v: usize = input.next();
		graph.add_directed_edge(u - 1, v - 1);
	}

	let mut dfs_space = crate::DFSSpace::new(&graph);
	let mut top_sort = dfs_space.topological_sort(&graph);
	top_sort.reverse();


	// Print the sorted nodes
	println!("{}", top_sort
		.iter()
		.map(|&x| (x + 1).to_string())
		.collect::<Vec<_>>()
		.join(" ")
	);
}

/*
1>2
3>2
4>2
2>5
6>5
4>6
----
6 6
1 2
3 2
4 2
2 5
6 5
4 6

 */
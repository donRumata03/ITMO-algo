extern crate dfs_mst;

use dfs_mst::{
	Graph,
	DFSSpace,
	InputReader
};

fn main() {
	let mut input = InputReader::new();
	// let mut output = OutputWriter::new();

	let mut graph = Graph::from_stdin(&mut input, true);

	let mut dfs_space = crate::DFSSpace::new(&graph);
	let (top_sort, success) = dfs_space.topological_sort(&graph);

	if success {
		println!("{}", top_sort
			.iter()
			.map(|&x| (x + 1).to_string())
			.collect::<Vec<_>>()
			.join(" ")
		);
	} else {
		println!("{}", -1);
	}
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
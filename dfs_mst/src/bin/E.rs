//! Decomposing undirected graph into VERTEX biconnected components
//! While finding cut vertexes, also maintain stack of edges and for each
//! tree edge going out of a cut vertex
//! take all edges from stack until this edge (inclusive) and add them to component
//! For the root of dfs tree, add all edges from stack to component at each tree edge
//! (<=> which was white at the moment of reaching it from the root)
//! (do it even if the root is not a cut vertex)

extern crate dfs_mst;

use self::dfs_mst::{find_edge_biconnected_components, print_vec};
use self::dfs_mst::{Graph, DFSSpace, InputReader, Edge, VisitColor};


fn main() {
	let mut input = InputReader::new();
	// let mut output = OutputWriter::new();

	let graph = Graph::from_stdin(&mut input, false);
	let n = graph.vertexes();

	let mut dfs_space = DFSSpace::new(&graph);
	let (_cutting_points, vertex_biconnected_components) =
		dfs_space.find_cutting_points_with_components(&graph);

	// Print components of edges
	println!("{}", vertex_biconnected_components.component_list.len());
	print_vec(&vertex_biconnected_components.component_map.iter()
		.map(|&index| index + 1)
		.collect()
	);
}

/*
5 6
1 2
2 3
3 1
1 4
4 5
5 1
------
1-2
2-3
3-1
1-4
4-5
5-1
---------------------------
// With additional bridge connected to 3
6 7
1 2
2 3
3 1
1 4
4 5
5 1
3 6
------
// Swap 1 and 2 to start from a non-cut vertex
5 6
2 1
1 3
3 2
2 4
4 5
5 2
----
2-1
1-3
3-2
2-4
4-5
5-2
___________________-
// Parallel edges (also with swapped 1 and 2)
5 8
2 1
1 3
3 2
2 4
2 1
4 5
5 2
4 5
-----
2-1
1-3
3-2
2-4
2-1
4-5
5-2
4-5


 */

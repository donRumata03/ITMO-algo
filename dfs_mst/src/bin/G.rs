extern crate dfs_mst;

use dfs_mst::{
	Graph,
	DFSSpace,
	InputReader,
	solve_not_numbered_2sat
};

fn parse_signed_name(input: &str) -> (String, bool) {
	let mut chars = input.chars();
	let sign = chars.next().unwrap();
	let name = chars.as_str();
	(name.to_owned(), sign == '+')
}


fn main() {
	let mut input = InputReader::new();
	// let mut output = OutputWriter::new();

	let n: usize = input.next();
	let m: usize = input.next();
	// input.consume_until(|c| c.is_ascii_whitespace());
	let _ = input.next_line();

	let mut friends = Vec::new();

	for _ in 0..n {
		friends.push(input.next_line().trim().to_owned());
	}
	// dbg!(&friends);

	let mut relations = Vec::new();
	for _ in 0..m {
		let line = input.next_line().trim().to_owned();

		let mut parts = line.split_whitespace();
		let left = parse_signed_name(parts.next().unwrap());
		let arrow = parts.next().unwrap();
		let right = parse_signed_name(parts.next().unwrap());
		// Apparantly, there are self-loops in the input.
		// if left.0 == right.0 {
		// 	panic!();
		// }

		// A -> B <=> !A || B

		relations.push((left.0, !left.1, right.0, right.1));
	}



	if let Some((truthness, map)) = solve_not_numbered_2sat(&relations) {
		let mut inverse_map = vec![String::new(); map.len()];
		for (name, i) in map {
			inverse_map[i] = name;
		}

		let mut result = Vec::new();
		for (i, &is_true) in truthness.iter().enumerate() {
			if is_true {
				result.push(inverse_map[i].to_owned());
			}
		}

		println!("{}", result.len());
		println!("{}", result.join("\n"));
	} else {
		println!("-1");
	}
}

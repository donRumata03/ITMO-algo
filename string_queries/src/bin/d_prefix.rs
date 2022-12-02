extern crate string_queries;

use string_queries::{
	InputReader,
	print_vec,
	prefix_function
};

fn main() {
	let mut input = InputReader::new();

	let p: String = input.next();
	let t: String = input.next();
	let combo = p.clone() + "#" + &t;
	let prefix = prefix_function(&combo);

	let mut result = Vec::new();
	for i in p.len() + 1..combo.len() {
		if prefix[i] == p.len() {
			result.push(i - p.len() - p.len() + 1);
		}
	}

	println!("{}", result.len());
	print_vec(&result);
}

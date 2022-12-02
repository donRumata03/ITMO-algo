extern crate string_queries;

use string_queries::{
	InputReader,
	print_vec,
	StringHasher
};

fn main() {
	let mut input = InputReader::new();

	let p: String = input.next();
	let t: String = input.next();
	let hasher_p = StringHasher::new(&p);
	let hasher_t = StringHasher::new(&t);


	let mut result = Vec::new();
	for i in 0_isize..t.len() as isize - p.len() as isize + 1 {
		if hasher_p.substring_hash(0, p.len()) == hasher_t.substring_hash(i as usize, i as usize + p.len()) {
			result.push(i + 1);
		}
	}

	println!("{}", result.len());
	print_vec(&result);
}

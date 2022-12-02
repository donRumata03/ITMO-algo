extern crate string_queries;

use string_queries::{
	InputReader,
	print_vec,
	StringHasher
};

fn next_subrange<R>(input: &mut InputReader<R>) -> (usize, usize)
where
	R: std::io::Read
{
	let l: usize = input.next();
	let r: usize = input.next();
	(l, r)
}

fn main() {
	let mut input = InputReader::new();

	let string: String = input.next();
	let hasher = StringHasher::new(&string);
	let m: usize = input.next();

	for _ in 0..m {
		let (l1, r1) = next_subrange(&mut input);
		let (l2, r2) = next_subrange(&mut input);
		println!("{}", if hasher.substring_hash(l1 - 1, r1) == hasher.substring_hash(l2 - 1, r2) { "Yes" } else { "No" });
	}
}

extern crate string_queries;

use string_queries::{
	InputReader,
	print_vec,
	prefix_function,
	z_function
};


fn main() {
	let mut input = InputReader::new();

	let string: String = input.next();
	print_vec(&z_function(&string)[1..]);
}

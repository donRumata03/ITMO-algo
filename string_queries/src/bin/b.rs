extern crate string_queries;

use string_queries::{
	InputReader
};


fn main() {
	let mut input = InputReader::new();

	let n: usize = input.next();
	println!("{n}");
}
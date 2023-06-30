extern crate nt;

use nt::{
	InputReader,
	print_vec
};

fn main() {
	let mut input = InputReader::new();

	let mut n: u32 = input.next();

	let mut res = vec![];
	let mut d = 2;
	while d * d <= n {
		while n % d == 0 {
			n /= d;
			res.push(d);
		}
		d += 1;
	}
	if n != 1 {
		res.push(n);
	}

	print_vec(&res);
}

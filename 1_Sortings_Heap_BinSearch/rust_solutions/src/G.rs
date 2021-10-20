// #![feature(cmp_min_max_by)]
use std::cmp::{max, min};
use std::iter::Filter;
use std::slice::Iter;
use std::io;
use std::str;
use std::io::BufReader;


/// Reads white-space separated tokens one at a time.

pub struct Scanner<R> {
	reader: R,
	buffer: Vec<String>,
}

impl<R: io::BufRead> Scanner<R> {
	pub fn new(reader: R) -> Self {
		Self {
			reader,
			buffer: vec![],
		}
	}

	/// Use "turbofish" syntax token::<T>() to select data type of next token.
	///
	/// # Panics
	///
	/// Panics if there's an I/O error or if the token cannot be parsed as T.
	pub fn token<T: str::FromStr>(&mut self) -> T {
		loop {
			if let Some(token) = self.buffer.pop() {
				return token.parse().ok().expect("Failed parse");
			}
			let mut input = String::new();
			self.reader.read_line(&mut input).expect("Failed read");
			self.buffer = input.split_whitespace().rev().map(String::from).collect();
		}
	}
}

///////////////////////////////////////////////////////////////////////////////////////////////////


fn discrete_bin_search<Pred>(mut predicate: Pred, L: i64, R: i64) -> i64
	where Pred: FnMut(i64) -> bool
{
	let mut l = L;
	let mut r = R;

	while r > l + 1 {
		let m = (l + r) / 2;

		if predicate(m) {
			r = m;
		} else {
			l = m;
		}
	}

	r
}



fn main() {
	let mut scanner = Scanner::new(BufReader::new(io::stdin()));

	let n: i64 = scanner.token();
	let x: i64 = scanner.token();
	let y: i64 = scanner.token();

	let a = min(x, y);
	let b = max(x, y);


	let double_copies_left = n - 1;
	let time_for_double_copies = discrete_bin_search(|t: i64| {
		return t / a + t / b >= double_copies_left;
	}, -1, double_copies_left * b + 1);

	let answer = a + time_for_double_copies;
	println!("{}", answer);
}

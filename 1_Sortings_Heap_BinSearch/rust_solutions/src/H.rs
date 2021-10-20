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


fn float_bin_search<Pred>(mut predicate: Pred, L: f64, R: f64) -> f64
	where Pred: FnMut(f64) -> bool
{
	let mut l = L;
	let mut r = R;

	for _ in 0..200 {
		let m = (l + r) / 2.;

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

	let C: f64 = scanner.token();

	let x = float_bin_search(|x: f64| {
		return x * x + x.sqrt() >= C;
	}, 0., 1e11);

	println!("{:.15}", x);
}

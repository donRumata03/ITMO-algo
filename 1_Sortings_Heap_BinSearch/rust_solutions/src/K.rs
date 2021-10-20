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

	let n = scanner.token();
	let k = scanner.token();

	let mut a = Vec::new();
	for _ in 0..n {
		a.push(scanner.token::<i64>());
	}

	let min_possible_s = discrete_bin_search(|sm| {
		let mut segments_made: i64 = 0;
		let mut last_segment_sum: i64 = 0;
		for i in 0..n {
			// Take max with sum <= sm
			if last_segment_sum + a[i] > sm {
				if a[i] > sm {
					return false;
				}

				segments_made += 1;
				last_segment_sum = a[i];
			} else {
				last_segment_sum += a[i];
			}
		}
		if last_segment_sum != 0 {
			segments_made += 1;
		}

		segments_made <= k
	}, -1, a.iter().sum::<i64>() + 1);

	println!("{}", min_possible_s);
}

/*

10 4
1 3 2 4 10 8 4 2 5 3


 */
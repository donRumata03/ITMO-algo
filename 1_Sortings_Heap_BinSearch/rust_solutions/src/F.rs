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


fn discrete_bin_search<Pred>(mut predicate: Pred, L: isize, R: isize) -> isize
	where Pred: FnMut(isize) -> bool
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


fn find_closest(ms: &Vec<i64>, query: i64) -> i64 {
	let min_geq_index = discrete_bin_search(|index| ms[index as usize] >= query, -1, ms.len() as isize) as i64;

	ms[
		*[min_geq_index - 1, min_geq_index].iter()
			.filter(|&&e| 0 <= e && e < ms.len() as i64)
			.min_by_key(|&&index| (ms[index as usize] - query).abs()).unwrap() as usize
	]
}

fn main() {
	let mut scanner = Scanner::new(BufReader::new(io::stdin()));

	let n: usize = scanner.token();
	let k: usize = scanner.token();

	let mut ms = Vec::new();
	for i in 0..n {
		ms.push(scanner.token());
	}

	let mut queries = Vec::new();
	for i in 0..k {
		queries.push(scanner.token::<i64>());
	}

	for q in queries {
		println!("{}", find_closest(&ms, q));
	}
}
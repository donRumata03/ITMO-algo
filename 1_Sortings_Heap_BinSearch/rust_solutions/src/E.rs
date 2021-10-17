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



struct FastSearcher {
	sorted_array: Vec<i32>
}

impl FastSearcher {
	fn new(ms: Vec<i32>) -> FastSearcher {
		let mut res = FastSearcher {
			sorted_array: ms
		};
		res.sorted_array.sort();

		res
	}

	/**
		How many elements are there in [l..r]
	*/
	fn points_in_segment(&self, l: i32, r: i32) -> usize {
		let first_index_geq_l = discrete_bin_search(
			|index| self.sorted_array[index as usize] >= l, -1, self.sorted_array.len() as isize
		);

		let first_index_greater_than_r = discrete_bin_search(
			|index| self.sorted_array[index as usize] > r, -1, self.sorted_array.len() as isize
		);

		(first_index_greater_than_r - first_index_geq_l) as usize
	}
}



fn main() {
	let mut scanner = Scanner::new(BufReader::new(io::stdin()));

	let n: usize = scanner.token();

	let mut ms = Vec::new();
	for _ in 0..n {
		ms.push(scanner.token());
	}
	let searcher = FastSearcher::new(ms);

	let k: usize = scanner.token();
	for _ in 0..k {
		let (l, r) = (scanner.token(), scanner.token());

		println!("{}", searcher.points_in_segment(l, r));
	}
}
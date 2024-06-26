use std::mem;

use std::io;
use std::str;
use std::io::BufReader;


fn merge_with_inversions(l: Vec<i64>, r: Vec<i64>) -> (Vec<i64>, i64) {
	let mut res = Vec::with_capacity(l.len() + r.len());
	let mut i = 0 as usize;
	let mut j = i;

	let mut inversions: i64 = 0;

	while i + j < l.len() + r.len() {
		if i != l.len() && (j == r.len() || l[i] <= r[j]) {
			res.push(l[i]);
			i += 1;
			inversions += j as i64;
		}
		else {
			res.push(r[j]);
			j += 1;
		}
	}

	(res, inversions)
}

fn inversions_by_merge_sort(v: Vec<i64>) -> (i64, Vec<i64>) {
	let n = v.len();
	if n <= 1 {
		return (0, v);
	}

	let m = n / 2;

	let (inv_l, sorted_l) = inversions_by_merge_sort((&v[..m]).to_vec());
	let (inv_r, sorted_r) = inversions_by_merge_sort((&v[m..]).to_vec());

	let (sorted, lr_inversions) = merge_with_inversions(sorted_l, sorted_r);

	(inv_l + lr_inversions + inv_r, sorted)
}


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


fn main() {
	let mut scanner = Scanner::new(BufReader::new(io::stdin()));


	let n: usize = scanner.token();
	let mut data = Vec::with_capacity(n);

	for _ in 0..n {
		data.push(scanner.token());
	}

	let (inversions, sorted) = inversions_by_merge_sort(data);

	println!("{}", inversions);
}

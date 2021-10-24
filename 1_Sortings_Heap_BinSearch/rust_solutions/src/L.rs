// #![feature(cmp_min_max_by)]
use std::cmp::{max, min};
use std::iter::Filter;
use std::slice::Iter;
use std::io;
use std::str;
use std::io::BufReader;
// use core::panicking::panic_fmt;


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


fn discrete_bin_search<Pred>(mut predicate: Pred, L: i64, R: i64) -> (i64, i64)
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

	(l, r)
}

///     //////////////////////////////////////////////////////////////////////////////////////////

fn mt_by_ind(i: u64, j: u64) -> u64 {
	(i + 1) * (j + 1)
}

fn get_border_in_table<P, G>(increasing_predicate: P, n: u64, v_gen: G) -> Vec<u64>
	where P: Fn(u64) -> bool, G: Fn(u64, u64) -> u64
{
	let mut border = Vec::with_capacity(n as usize);
	let mut y = n;
	for x in 0..n {
		while y > 0 && increasing_predicate(v_gen(x, y - 1)) {
			y -= 1;
		}
		border.push(
			y
		);
	}

	border
}

fn first_index_of_cell_in_table<G>(v: u64, n: u64, v_gen: G) -> u64
	where G: Fn(u64, u64) -> u64
{
	// let v = v_gen(i, j);
	// There are v - 1 numbers guaranteed to be smaller than v and probably — some other ones

	let border = get_border_in_table(|vr| vr >= v, n, v_gen);
	// println!("{:?}", border);

	border.iter().sum()
}

fn cell_with_index(index: u64, n: u64) -> u64 {
	let policeman_squares = discrete_bin_search(
		|s| first_index_of_cell_in_table(mt_by_ind(s as u64, s as u64), n, mt_by_ind) >= index,
		-1, n as i64
	);

	let (lp, rp) = policeman_squares;
	let lp = lp.max(0);

	let lower_bound = get_border_in_table(|p| p >= mt_by_ind(lp as u64, lp as u64), n, mt_by_ind);
	let upper_bound = get_border_in_table(|p| p > mt_by_ind(rp as u64, rp as u64), n, mt_by_ind);

	let mut numbers_considered = Vec::new();
	for x in 0..n {
		for y in lower_bound[x as usize]..upper_bound[x as usize] {
			numbers_considered.push(mt_by_ind(x, y));
		}
	}
	numbers_considered.sort();
	let first_ind = first_index_of_cell_in_table(numbers_considered[0], n, mt_by_ind);

	return numbers_considered[(index - first_ind) as usize];
	// panic!();
}

fn main() {
	let mut scanner = Scanner::new(BufReader::new(io::stdin()));

	let n: u64 = scanner.token();
	let k: u64 = scanner.token();

	// println!("{}", first_index_of_cell_in_table(2, n, mt_by_ind));

	// println!("{:?}", get_border_in_table(|v| v > 20, 7, mt_by_ind));

	println!("{}", cell_with_index(k - 1, n));
}
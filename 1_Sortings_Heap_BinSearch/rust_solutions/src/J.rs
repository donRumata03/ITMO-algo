// J. K-best

// #![feature(cmp_min_max_by)]
use std::cmp::{max, min};
use std::iter::Filter;
use std::slice::Iter;
use std::io;
use std::str;
use std::io::BufReader;
use std::str::FromStr;
use std::num::ParseIntError;


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


fn float_bin_search_left<Pred>(mut predicate: Pred, L: f64, R: f64) -> f64
	where Pred: FnMut(f64) -> bool
{
	let mut l = L;
	let mut r = R;

	for _ in 0..50 {
		let m = (l + r) / 2.;

		if predicate(m) {
			r = m;
		} else {
			l = m;
		}
	}

	l
}

///     //////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, Clone)]
struct Jewel {
	value: i64,
	weight: i64,
	index: usize
}

// impl FromStr for Point {
// 	type Err = ParseIntError;
//
// 	fn from_str(s: &str) -> Result<Self, Self::Err> {
// 		let x_fromstr = coords[0].parse::<i32>()?;
// 		let y_fromstr = coords[1].parse::<i32>()?;
//
// 		Ok(Point { x: x_fromstr, y: y_fromstr })
// 	}
// }

fn largest_set_for_c(c: f64, jewelery: &Vec<Jewel>, fixed_k: Option<usize>) -> Vec<Jewel> {
	let jewel_cost = |j: &Jewel| j.value as f64 - j.weight as f64 * c;

	let mut js = jewelery.clone();
	js.sort_by(|a, b| jewel_cost(b).partial_cmp(&jewel_cost(a)).unwrap());

	let mut cur_sum = 0.;

	let res = match fixed_k {
		None => js.iter().take_while(| &j | { cur_sum += jewel_cost(j); cur_sum >= 0. }).cloned().collect(),
		Some(k) => js.iter().take(k).cloned().collect()
	};

	// println!("{}", c);
	// println!("{:?}", js);
	// println!("{:?}", js.iter().map(jewel_cost).collect::<Vec<_>>());
	// println!("{:?}", res);
	// println!("_______________________________");

	res
}

fn main() {
	let mut scanner = Scanner::new(BufReader::new(io::stdin()));

	let n: usize = scanner.token();
	let k: usize = scanner.token();

	let mut jewelery = Vec::new();
	for i in 0..n {
		jewelery.push(Jewel {
			value: scanner.token(),
			weight: scanner.token(),
			index: i
		});
	}

	let l = float_bin_search_left(|c| {
		largest_set_for_c(c, &jewelery, None).len() < k
	}, 0., jewelery.iter().map(|j| j.value as f64).sum::<f64>() * 12f64);

	let mut best_set = largest_set_for_c(l, &jewelery, Some(k)).iter().map(|j| j.index).collect::<Vec<_>>();
	best_set.sort();


	println!("{}", best_set.iter().map(|j| (j + 1).to_string()).collect::<Vec<String>>().join("\n"));
}
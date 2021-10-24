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

	/// Use "turbofish" syntax `token::<T>()` to select data type of next token.
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


fn float_ternary_search<Pred>(mut min_obj: Pred, L: f64, R: f64) -> f64
	where Pred: FnMut(f64) -> f64
{
	let mut l = L;
	let mut r = R;

	for _ in 0..200 {
		let ml = (2. * l + r) / 3.;
		let mr = (l + 2. * r) / 3.;

		if min_obj(ml) > min_obj(mr) {
			l = ml;
		} else {
			r = mr;
		}
	}

	r
}



fn main() {
	let mut scanner = Scanner::new(BufReader::new(io::stdin()));

	let V_p = scanner.token::<i32>() as f64;
	let V_f = scanner.token::<i32>() as f64;

	let a: f64 = scanner.token();

	let x = float_ternary_search(|x: f64| {
		let dist_p = ((1. - a).powi(2) + x.powi(2)).sqrt();
		let dist_f = (a.powi(2) + (1. - x).powi(2)).sqrt();

		dist_p / V_p + dist_f / V_f
	}, 0., 1.);

	println!("{:.15}", x);
}

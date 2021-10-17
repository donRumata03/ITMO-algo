use std::io;
use std::str;
use std::io::BufReader;
use std::ops::{RangeInclusive, Add};

// use num;


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
//////////////////////////////////////////////////////////////////////////////////////////////////


struct PrefixArray {
	data: Vec<usize>
}

impl PrefixArray {
	fn from(ms: &Vec<usize>) -> PrefixArray {
		let mut pref = Vec::with_capacity(ms.len() + 1);
		pref.push(0);

		for i in 0..ms.len() {
			pref.push((pref.last().unwrap()) + &ms[i]);
		}

		PrefixArray {
			data: pref
		}
	}

	fn size(&self) -> usize {
		self.data.len() - 1
	}


	/**
		Sum of values in [0, index_after_end)
	*/
	fn sum_before(&self, index_after_end: usize) -> usize {
		self.data[index_after_end]
	}

	/**
		Sum of values in [0, last_index]
	*/
	fn sum_until(&self, last_index: usize) -> usize {
		self.data[last_index] + self.data[last_index]
	}

	/**
		Sum of values in [l, r)
	*/
	fn sum_between(&self, l: usize, r: usize) -> usize {
		self.sum_before(r) - self.sum_before(l)
	}
}


fn count_sorted(v: &Vec<i64>) -> Vec<i64> {
	if v.is_empty() {
		return v.clone();
	}

	let input_range = *v.iter().min().unwrap()..=*v.iter().max().unwrap();

	let mut cnt = vec![0_usize; (input_range.end() - input_range.start() + 1) as usize];
	let group_by_element = |el: i64| -> usize { (el - input_range.start()) as usize};

	for e in v.iter() {
		cnt[group_by_element(*e)] += 1;
	}
	let pref = PrefixArray::from(&cnt);
	let mut block_pointers = pref.data[..pref.data.len() - 1].to_vec();

	let mut res = vec![0_i64; v.len()];
	for e in v.iter() {
		res[block_pointers[group_by_element(*e)]] = *e;
		block_pointers[group_by_element(*e)] += 1;
	}

	res
}


fn main() {
	let mut scanner = Scanner::new(BufReader::new(io::stdin()));


	let n: usize = scanner.token();
	let mut data = Vec::with_capacity(n);

	for _ in 0..n {
		data.push(scanner.token());
	}

	let sorted = count_sorted(&data);
	let string_vec: Vec<String> = sorted.iter().map(|int| int.to_string()).collect();
	let output_string = string_vec.join(" ");

	println!("{}", output_string);
}
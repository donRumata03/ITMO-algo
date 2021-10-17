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




struct MaxHeap {
	data: Vec<i64>
}

impl MaxHeap {
	fn left_son(index: usize) -> usize {
		index * 2 + 1
	}
	fn right_son(index: usize) -> usize {
		index * 2 + 2
	}
	fn parent(index: usize) -> usize {
		(index - 1) / 2
	}
	fn existing_children(&self, index: usize) -> Vec<usize> {
		[MaxHeap::left_son(index), MaxHeap::right_son(index)]
			.iter()
			.filter(|c| -> bool { c < &&self.data.len() }).map(|v: &usize| *v).collect()
	}

	fn sift_up(&mut self, index: usize) {
		let mut cur_pos = index;

		// Swap with parent while it's smaller:
		while cur_pos != 0 && self.data[cur_pos] > self.data[MaxHeap::parent(cur_pos)] {
			self.data.swap(cur_pos, MaxHeap::parent(cur_pos));
			cur_pos = MaxHeap::parent(cur_pos);
		}
	}

	fn sift_down(&mut self, index: usize) {
		let mut cur_pos = index;

		loop {
			let ex_children = self.existing_children(cur_pos);
			if ex_children.is_empty() { break; }

			let max_child = *ex_children.iter()
				.min_by(|&&i1, &&i2| self.data[i2].cmp(&self.data[i1]))
				.unwrap();

			if self.data[cur_pos] >= self.data[max_child] {
				break;
			}

			self.data.swap(cur_pos, max_child);
			cur_pos = max_child;
		}
	}




	fn new(ms: Vec<i64>) -> MaxHeap {
		let mut res = MaxHeap { data: ms };
		for i in (0..res.data.len()).rev() {
			res.sift_down(i);
		}

		res
	}

	fn insert(&mut self, el: i64) {
		self.data.push(el);
		self.sift_up(self.data.len() - 1);
	}

	fn extract_max(&mut self) -> i64 {
		let max_el = *self.data.first().unwrap();
		let last_index = self.data.len() - 1;
		self.data.swap(0, last_index);
		self.data.pop();
		self.sift_down(0);

		max_el
	}
}

fn process_queries(initial_values: Vec<i64>, qs: &Vec<Query>) {
	let mut h = MaxHeap::new(initial_values);

	qs.iter().for_each(|query| match query {
		Query::Insert(el) => h.insert(*el),
		Query::Extract => println!("{}", h.extract_max())
	});
}


enum Query {
	Insert(i64),
	Extract
}

fn main() {
	let mut scanner = Scanner::new(BufReader::new(io::stdin()));

	let n: usize = scanner.token();

	let mut qs = Vec::with_capacity(n);
	for _ in 0..n {
		qs.push(match scanner.token::<usize>() {
			0 => Query::Insert(scanner.token::<i64>()),
			_ => Query::Extract
		});
	}

	process_queries(Vec::new(), &qs);
}
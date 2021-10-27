// H. K-th sum

// #![deny(deprecated)]


// #![feature(cmp_min_max_by)]
use std::cmp::{max, min};
use std::iter::Filter;
use std::slice::Iter;
use std::io;
use std::str;
use std::io::BufReader;
use std::collections::HashMap;
// use rand::Rng;
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

struct CompressedArray<T: Ord> {
	data: Vec<(T, usize)>
}

impl<T: Ord> CompressedArray<T> {
	fn new(mut ms: Vec<T>) -> CompressedArray<T> {
		let mut res_data: Vec<(T, usize)> = Vec::new();

		ms.sort();
		for v in ms {
			if res_data.is_empty() || res_data.last().unwrap().0 != v {
				res_data.push((v, 1));
			} else {
				let (ref mut val, ref mut count) = res_data.last_mut().unwrap();
				*count += 1;
			}
		}

		CompressedArray {
			data: res_data
		}
	}
}

///     //////////////////////////////////////////////////////////////////////////////////////////

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
	let border = get_border_in_table(|vr| vr >= v, n, v_gen);
	// println!("First index of {} is {} (border: {:?})", v, border.iter().sum::<u64>(), border);

	border.iter().sum()
}

fn last_index_of_cell_in_table<G>(v: u64, n: u64, v_gen: G) -> u64
	where G: Fn(u64, u64) -> u64
{
	let border = get_border_in_table(|vr| vr > v, n, v_gen);
	// println!("Last index of {} is {} (border: {:?})", v, border.iter().sum::<u64>() - 1u64, border);

	border.iter().sum::<u64>() - 1
}

fn kth_sum(index: u64, n: u64, mut a: Vec<u64>, mut b: Vec<u64>) -> u64 {
	a.sort();
	b.sort();

	let mt_by_ind = |i: u64, j: u64| {
		// if i == n || j == n {panic!();}
		if i as usize >= a.len() || j as usize >= b.len() {loop{}};
		a[i as usize] + b[j as usize]
	};

	let policeman_squares = discrete_bin_search(
		|s| first_index_of_cell_in_table(mt_by_ind(s as u64, s as u64), n, mt_by_ind) >= index,
		-1, n as i64
	);

	let (lp, rp) = policeman_squares;
	let lp = lp.max(0);
	let rp = rp.min(n as i64 - 1);

	let lower_bound = get_border_in_table(|p| p >= mt_by_ind(lp as u64, lp as u64), n, mt_by_ind);
	let upper_bound = get_border_in_table(|p| p > mt_by_ind(rp as u64, rp as u64), n, mt_by_ind);

	let mut numbers_considered = HashMap::new();
	for x in 0..n {
		for y in lower_bound[x as usize]..upper_bound[x as usize] {
			*numbers_considered.entry(mt_by_ind(x, y)).or_insert(0usize) += 1;
		}
	}
	let mut num_pairs: Vec<(u64, usize)> = numbers_considered.into_iter().collect();
	num_pairs.sort_by_key(|v| v.0);
	let first_ind = first_index_of_cell_in_table(num_pairs[0].0, n, mt_by_ind);

	// return numbers_considered[(index - first_ind) as usize];

	let ind_required = (index - first_ind) as usize;
	let mut ind_before = 0;
	for (num, count) in num_pairs {
		// num's range is: [ind_before, ind_before + count)
		if ind_required < ind_before + count { return num; }

		ind_before += count;
	}
	panic!();

	// match numbers_considered.get((index - first_ind) as usize) {
	// 	None => { loop {} }
	// 	Some(&v) => v
	// }
	// panic!();
}

#[deprecated]
fn optimized_kth_sum(index: u64, n: u64, mut a: Vec<u64>, mut b: Vec<u64>) -> u64 {
	a.sort();
	b.sort();

	let mt_by_ind = |i: u64, j: u64| {
		a[i as usize] + b[j as usize]
	};


	// Look for an appropriate col:
	let appropriate_col = discrete_bin_search(|col| {
		index <= last_index_of_cell_in_table(mt_by_ind(col as u64, n - 1), n, mt_by_ind)
	}, -1, n as i64).1;
	println!("Column: {}", appropriate_col);

	let appropriate_row = discrete_bin_search(|row| {
		index <= last_index_of_cell_in_table(mt_by_ind(appropriate_col as u64, row as u64), n, mt_by_ind)
	}, -1, n as i64).1;
	println!("Row: {}", appropriate_row);

	return mt_by_ind(appropriate_col as u64, appropriate_row as u64);
}
fn extremely_optimized_kth_sum(index: u64, n: u64, mut a: Vec<u64>, mut b: Vec<u64>) -> u64 {
	a.sort();
	b.sort();

	let mt_by_ind = |i: u64, j: u64| {
		a[i as usize] + b[j as usize]
	};


	// Look for an appropriate col:
	let answer = discrete_bin_search(|val| {
		index <= last_index_of_cell_in_table(val as u64, n, mt_by_ind)
	}, (mt_by_ind(0, 0)) as i64 - 1, (mt_by_ind(n - 1, n - 1) + 1) as i64).1;

	answer as u64
}

fn print_last_index_of(val: usize) {
	let all_values = vec![3, 5, 5, 5, 7, 7, 7, 7, 9, 9, 9, 9, 9, 11, 11, 11, 11, 11, 13, 13, 13, 13, 15, 15, 17];
	// println!("Last index of {} is: {:#?}", val, all_values.iter().rposition(|&v| v == val).unwrap());
}

// #[deprecated]
// fn stress_test(n: usize) {
// 	let mut rng = rand::thread_rng();
// 	let mut rng2 = rand::thread_rng();
//
// 	let mut gen = || (0..n).map(|_| rng.gen_range(0..10u64)).collect::<Vec<_>>();
//
// 	loop {
// 		let a = gen();
// 		let b = gen();
//
// 		let index = rng2.gen_range(0..n.pow(2));
// 		let true_answer = kth_sum(index as u64, n as u64, a.clone(), b.clone());
// 		let candidate_answer = optimized_kth_sum(index as u64, n as u64, a.clone(), b.clone());
//
// 		if candidate_answer != true_answer {
// 			println!("False answer found!");
// 			println!("n was: {}", n);
// 			println!("a was: {:?}", a);
// 			println!("b was: {:?}", b);
// 			println!("Index (0-based) was: {} ({} for 1-based)", index, index + 1);
//
// 			println!("True answer: {}", true_answer);
// 			println!("Optimized answer: {}", candidate_answer);
//
// 			return;
// 		}
// 	}
// }



fn main() {
	// stress_test(2);
	// return;

	let mut scanner = Scanner::new(BufReader::new(io::stdin()));

	let n: u64 = scanner.token();
	let k: u64 = scanner.token();

	let mut a = Vec::new();
	let mut b = Vec::new();

	for _ in 0..n {
		a.push(scanner.token());
	}
	for _ in 0..n {
		b.push(scanner.token());
	}

	// let c_a = CompressedArray::new(a);
	// let c_b = CompressedArray::new(b);

	// println!("{}", first_index_of_cell_in_table(2, n, mt_by_ind));

	// println!("{:?}", get_border_in_table(|v| v > 20, 7, mt_by_ind));


	// println!("{}", optimized_kth_sum(k - 1, n, a.clone(), b.clone()));
	println!("{}", extremely_optimized_kth_sum(k - 1, n, a.clone(), b.clone()));
	// println!("{}", kth_sum(k - 1, n, a.clone(), b.clone()));

	// println!("_____________test");
	// print_last_index_of(9);
	// print_last_index_of(11);
	// print_last_index_of(13);
	// print_last_index_of(15);
	// print_last_index_of(17);
}

/*
5 25
4 2 6 4 8
7 3 1 9 5


3 5
1 1 1
1 1 1


False answer found!
n was: 2
a was: [5, 7]
b was: [0, 9]
Index (0-based) was: 1 (2 for 1-based)
True answer: 7
Optimized answer: 14
2 2
5 7
0 9

*/

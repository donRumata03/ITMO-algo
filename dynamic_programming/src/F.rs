#![allow(dead_code)]
#![allow(unused_imports)]

use {
	std::{
		io::{
			self,
			Read,
			Write,
			Stdin,
			Stdout,
			BufReader
		},
		fmt::{Display},
		str,
		cmp::min,
		iter::once,
		fs::{File, OpenOptions}
	}
};
use std::collections::{HashMap, HashSet};
use std::cmp::{max};
use std::borrow::Borrow;
use std::cell::UnsafeCell;
use std::ops::{Deref, Index, Add, Sub, AddAssign};
use std::mem::size_of;
use std::io::{BufRead, repeat};
// use itertools::{iproduct, Itertools};
use std::iter::Sum;
use std::fmt::Formatter;
// use itertools::enumerate;


/// Writer
pub struct OutputWriter<W: Write> {
	writer: W,
	buf: Vec<u8>,
}

impl OutputWriter<Stdout> {
	pub fn new() -> Self { Self::from_writer(io::stdout()) }
}

impl OutputWriter<File> {
	pub fn from_file(path: &str) -> Self {
		let file = OpenOptions::new()
			.write(true)
			.create(true)
			.open(path);
		Self::from_writer(file.unwrap())
	}
}

impl<W: Write> OutputWriter<W> {
	pub fn from_writer(writer: W) -> Self {
		let buf = Vec::with_capacity(1 << 16);
		Self { writer, buf }
	}

	pub fn print<T: Display>(&mut self, t: T) {
		write!(self, "{}", t).unwrap();
	}

	pub fn prints<T: Display>(&mut self, t: T) {
		write!(self, "{} ", t).unwrap();
	}

	pub fn println<T: Display>(&mut self, t: T) {
		writeln!(self, "{}", t).unwrap();
	}
}

impl<W: Write> Write for OutputWriter<W> {
	fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
		self.buf.extend(bytes);
		Ok(bytes.len())
	}

	fn flush(&mut self) -> std::io::Result<()> {
		self.writer.write_all(&self.buf)?;
		self.writer.flush()?;
		self.buf.clear();
		Ok(())
	}
}

impl<W: Write> Drop for OutputWriter<W> {
	fn drop(&mut self) { self.flush().unwrap(); }
}

macro_rules! EOF {
    // `()` indicates that the macro takes no argument.
    () => {
        // The macro will expand into the contents of this block.
        "InputReader: Reached end of input!"
    };
}

// const EOF: &'static str = ;

pub struct InputReader<R: Read> {
	reader: R,
	buf: Vec<u8>,
	bytes_read: usize,
	current_index: usize,
}

impl InputReader<Stdin> {
	pub fn new() -> Self {
		Self::from_reader(io::stdin())
	}
}

impl InputReader<File> {
	pub fn from_file(path: &str) -> Self {
		Self::from_reader(File::open(path).unwrap())
	}
}

impl<R: Read> InputReader<R> {
	pub fn from_reader(reader: R) -> Self {
		Self {
			reader,
			buf: vec![0; 1 << 16],
			bytes_read: 0,
			current_index: 0,
		}
	}

	pub fn next<T: InputReadable>(&mut self) -> T {
		T::from_input(self)
	}

	pub fn next_line(&mut self) -> String {
		assert!(self.has_more(), EOF!());
		let mut line = String::new();
		while self.peek() != '\n' {
			line.push(self.peek());
			self.consume();
			if !self.has_more() { break; }
		}
		self.consume(); // consume '\n'
		line
	}

	pub fn has_more(&mut self) -> bool {
		if self.current_index >= self.bytes_read {
			self.bytes_read = self.reader.read(&mut self.buf[..]).unwrap();
			self.current_index = 0
		}
		self.bytes_read > 0
	}

	pub fn set_buf_size(&mut self, buf_size: usize) {
		self.buf.resize(buf_size, 0);
	}

	fn peek(&self) -> char { self.buf[self.current_index] as char }

	fn consume(&mut self) { self.current_index += 1; }

	fn pop_digit(&mut self) -> u64 {
		let c = self.peek();
		self.consume();
		c as u64 - '0' as u64
	}

	fn consume_until<F: Fn(char) -> bool>(&mut self, test: F) {
		loop {
			assert!(self.has_more(), EOF!());
			if test(self.peek()) { return; }
			self.consume();
		}
	}

	fn consume_until_sign(&mut self) -> i64 {
		loop {
			self.consume_until(|c| c.is_ascii_digit() || c == '-');
			if self.peek() != '-' { return 1; }

			self.consume();
			assert!(self.has_more(), EOF!());
			if self.peek().is_ascii_digit() { return -1; }
		}
	}
}

pub trait InputReadable {
	fn from_input<R: Read>(input: &mut InputReader<R>) -> Self;
}

impl InputReadable for u64 {
	fn from_input<R: Read>(input: &mut InputReader<R>) -> Self {
		input.consume_until(|c| c.is_ascii_digit());
		let mut num = 0;
		while input.peek().is_ascii_digit() {
			num = num * 10 + input.pop_digit();
			if !input.has_more() { break; }
		}
		num
	}
}

impl InputReadable for i64 {
	fn from_input<R: Read>(input: &mut InputReader<R>) -> Self {
		let sign = input.consume_until_sign();
		u64::from_input(input) as i64 * sign
	}
}

impl InputReadable for f64 {
	fn from_input<R: Read>(input: &mut InputReader<R>) -> Self {
		let sign = input.consume_until_sign() as f64;
		let mut num = 0.0;
		while input.peek().is_ascii_digit() {
			num = num * 10.0 + input.pop_digit() as f64;
			if !input.has_more() { break; }
		}

		let mut factor = 1.0;
		if input.peek() == '.' {
			input.consume();
			while input.has_more() && input.peek().is_ascii_digit() {
				num = num * 10.0 + input.pop_digit() as f64;
				factor *= 10.0;
			}
		}
		sign * num / factor
	}
}

impl InputReadable for String {
	fn from_input<R: Read>(input: &mut InputReader<R>) -> Self {
		input.consume_until(|c| c.is_ascii_graphic());
		let mut word = String::new();
		while input.peek().is_ascii_graphic() {
			word.push(input.peek());
			input.consume();
			if !input.has_more() { break; }
		}
		word
	}
}

impl InputReadable for char {
	fn from_input<R: Read>(input: &mut InputReader<R>) -> Self {
		input.consume_until(|c| c.is_ascii_graphic());
		let c = input.peek();
		input.consume();
		c
	}
}

macro_rules! impl_readable_from {
  ($A:ty, [$($T:ty),+]) => {
    $(impl InputReadable for $T {
      fn from_input<R: Read>(input: &mut InputReader<R>) -> Self {
        <$A>::from_input(input) as $T
      }
    })+
  };
}
impl_readable_from!{ u64, [u32, u16, u8, usize] }
impl_readable_from!{ i64, [i32, i16, i8, isize] }
impl_readable_from!{ f64, [f32] }
//////////////////////////////////////////////////////////////////////////////////////////////////

fn option_if<T>(value: T, condition: bool) -> Option<T> {
	if condition { Some(value) } else { None }
}

fn option_if_with<T, Gen>(f: Gen, condition: bool) -> Option<T>
	where Gen: Fn() -> T
{
	if condition { Some(f()) } else { None }
}

fn optimize_food(prices: Vec<usize>) -> (usize, Vec<usize>, usize, usize) {
	let days = prices.len();

	/// `dp[c]` at day `t`: smallest sum spent with `c` coupons unused
	/// dp[c] -> new_dp[c - 1] if c > 0; -> `new_dp[c (+ 1 if cost > 100)?] + cost`
	/// // either (use) or (pay and get coupon if should pay enough)
	let mut dp = vec![Some(0_usize)];
	let mut use_coupon = vec![];

	for d in 0..days {
		let mut this_use_coupon = vec![false; dp.len() + 1];
		let mut new_dp = vec![None; dp.len() + 1];

		for (prev_coupons, prev_cost) in dp.iter()
			.enumerate()
			.filter(|&(i, v)| v.is_some())
			.map(|(i, v)| (i, v.unwrap()))
		{
			option_if_with(|| (true, prev_coupons - 1, prev_cost), prev_coupons > 0).into_iter()
				.chain(once(
					(false, prev_coupons + if prices[d] > 100 {1} else {0}, prev_cost + prices[d])
				))
			.for_each(|(use_coupon, c, opt_cost)| {
				if new_dp[c].is_none() || opt_cost < new_dp[c].unwrap() {
					new_dp[c] = Some(opt_cost);
					this_use_coupon[c] = use_coupon;
				}
			});
		}

		use_coupon.push(this_use_coupon);
		dp = new_dp;
	}

	let (best_coupons, min_cost) = dp.into_iter()
		.enumerate()
		.filter(|&(_, v)| v.is_some())
		.map(|(i, v)| (i, v.unwrap()))
		.min_by_key(|&(_, v)| v)
		.unwrap();

	let mut to_buy = Vec::new();
	let mut applied_coupons = 0;
	let mut current_coupons = best_coupons;
	for d in (0..days).rev() {
		if use_coupon[d][current_coupons] {
			current_coupons += 1;
			applied_coupons += 1;
			to_buy.push(d);
		} else {
			if prices[d] > 100 { current_coupons -= 1; };
		}
	}
	assert_eq!(current_coupons, 0);

	(min_cost, to_buy.into_iter().rev().collect(), best_coupons, applied_coupons)
}

fn main() {
	let mut input = InputReader::new();
	let mut output = OutputWriter::new();


	let n = input.next();

	let s = input.next_line();

	let mut costs = Vec::new();
	for _ in 0..n {
		costs.push(input.next());
	}

	let (opt_cost, days, left_coupons, all_coupons) = optimize_food(costs);

	println!("{}", opt_cost);
	println!("{} {}", left_coupons, all_coupons);
	println!("{}", days.iter().map(|&d| (d + 1).to_string()).collect::<Vec<_>>().join("\n"));
}
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
use std::iter::Sum;
use std::fmt::Formatter;
use std::collections::hash_map::Entry;


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

fn circular(n: usize) -> impl Iterator<Item=(usize, usize)> {
	(0..n).zip((0..n).cycle().skip(1))
}

fn consec(n: usize) -> impl Iterator<Item=(usize, usize)> {
	(0..n).zip(1..n)
}
fn ones(msk: usize) -> impl Iterator<Item=usize> {
	(0..std::mem::size_of::<usize>() * 8).filter(move |&i| msk & (1 << i) != 0)
}

fn get_non_equal_indexes<T>(vec: &mut Vec<T>, i: usize, j: usize) -> (&mut T, &mut T) {
	assert_ne!(i, j);

	let ith: &mut T;
	let jth: &mut T;
	unsafe {
		ith = (vec.as_mut_ptr().offset(i as isize)).as_mut().unwrap();
		jth = (vec.as_mut_ptr().offset(j as isize)).as_mut().unwrap();
	}

	(ith, jth)
}


struct BitSet {
	mask: usize
}
impl BitSet {
	fn from(msk: usize) -> BitSet {
		BitSet {
			mask: msk
		}
	}


	fn get(&self, index: usize) -> bool {
		(self.mask & (1 << index)) != 0
	}


	fn set_to(&mut self, position: usize, value: bool) {
		self.mask = (self.mask & !(1 << position)) | ((value as usize) << position);
	}

	fn set_true(&mut self, position: usize) {
		self.mask |= (1_usize) << position;
	}
	fn set_false(&mut self, position: usize) {
		self.mask &= !((1_usize) << position);
	}
}

//////////////////////////////////////////////////////////////////////////////////////////////////


fn main() {
	let mut input = InputReader::new();
	let mut _output = OutputWriter::new();

	let n: usize = input.next();
	let m: usize = input.next();

	let (Y, X) = (min(n, m), max(n, m));
	let prof_len = Y as u32 + 1;
	let profiles = 2usize.pow(prof_len);

	let mut dp = vec![0_u64; profiles];
	for i in 1..profiles / 2 {
		dp[i as usize] = 1;
	}

	for x in 0..X {
		for y in 0..Y {
			let mut new_dp = vec![0_u64; profiles];

			for prof in 0..profiles {
				if y == 0 {
					// Always both 0 and 1
					let mut new_prof_0 = BitSet::from(prof);
					new_prof_0.set_false(0);
					new_prof_0.set_to(prof_len as usize - 1, prof & 1 != 0);
					let new_prof_1 = new_prof_0.mask | 0b1;

					new_dp[new_prof_0.mask] += dp[prof];
					new_dp[new_prof_1] += dp[prof];
				} else {
					let lu = dp[prof] & (1 << (prof_len - 1)) != 0;
					let ru = dp[prof] & (1 << (y - 1)) != 0;
					let lb = dp[prof] & (1 << y) != 0;

					if [lu, ru, lb].iter().any(|&v| v != false) {
						// Can use false:
						let mut new_prof_0 = BitSet::from(prof);
						new_prof_0.set_to((prof_len - 1) as usize, lb);
						new_prof_0.set_to(y as usize, false);
						new_dp[new_prof_0.mask] += dp[prof];
					}

					if [lu, ru, lb].iter().any(|&v| v != true) {
						// Can use true:
						let mut new_prof_1 = BitSet::from(prof);
						new_prof_1.set_to((prof_len - 1) as usize, lb);
						new_prof_1.set_to(y as usize, true);
						new_dp[new_prof_1.mask] += dp[prof];
					}
				}
			}

			// dbg!(&dp);
			// dbg!(&new_dp);
			dp = new_dp;
		}
	}

	println!("{}", dp.iter().sum::<u64>());
}

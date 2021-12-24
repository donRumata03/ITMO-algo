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
use std::collections::HashMap;
use std::cmp::{max};
use std::borrow::Borrow;
use std::cell::UnsafeCell;
use std::ops::{Deref, Index, Add, Sub};
use std::mem::size_of;
use std::io::BufRead;
// use itertools::Itertools;


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




//////////////////////////////////////////////////////////////////////////////////////////////////

fn main() {
	let mut input = InputReader::new();
	let mut output = OutputWriter::new();

	let rows: usize = input.next();
	let cols: usize = input.next();

	let mut pole = Vec::new();
	for r in 0..rows {
		pole.push(Vec::new());
		for c in 0..cols {
			pole.last_mut().unwrap().push(input.next::<i64>());
		}
	}

	let mut dp = vec![vec![0i64; cols]; rows];
	let mut parent = vec![vec![(0usize, 0usize); cols]; rows];

	// for r in 0..rows { dp[r][0] = ?; }
	// for c in 0..cols { dp[0][c] = ?; }

	for r in 0..rows {
		for c in 0..cols {
			let best_ancestor = [(r as isize - 1, c as isize), (r as isize, c as isize - 1)]
				.iter().filter(|&(y, x)| *x >= 0 && *y >= 0)
				.map(|&(y, x)| (y as usize, x as usize))
				.max_by_key(|&(y, x)| dp[y][x]);

			dp[r][c] = pole[r][c] + best_ancestor.map(|(y, x)| dp[y][x]).unwrap_or_default();
			parent[r][c] = best_ancestor.unwrap_or_default();
		}
	}

	// dbg!(dp);
	// dbg!(parent);

	let mut path = Vec::new();
	let mut cursor = (rows - 1, cols - 1);
	while cursor != (0, 0) {
		path.push(cursor);
		cursor = parent[cursor.0][cursor.1].clone();
	}
	path.push((0, 0));
	path.reverse();

	println!("{}", dp[rows - 1][cols - 1]);
	for (i, &this) in path[..path.len() - 1].iter().enumerate() {
		let next = path[i + 1];
		print!("{}", if next.0 == this.0 + 1 {"D"} else {"R"});
	}
	println!();

}
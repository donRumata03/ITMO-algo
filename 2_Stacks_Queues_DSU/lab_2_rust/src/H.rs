// #[macro_use]
// extern crate derive_more;
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


const EOF: &'static str = "InputReader: Reached end of input!";

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
		assert!(self.has_more(), EOF);
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
			assert!(self.has_more(), EOF);
			if test(self.peek()) { return; }
			self.consume();
		}
	}

	fn consume_until_sign(&mut self) -> i64 {
		loop {
			self.consume_until(|c| c.is_ascii_digit() || c == '-');
			if self.peek() != '-' { return 1; }

			self.consume();
			assert!(self.has_more(), EOF);
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

#[inline]
#[must_use]
pub fn min_max_by_key<'a, T, F: FnMut(&T) -> K, K: Ord>(v1: &'a T, v2: &'a T, mut f: F) -> (&'a T, &'a T) {
	(
		*[&v1, &v2].iter().min_by_key(|&&&v| f(v)).unwrap(),
		*[&v1, &v2].iter().max_by_key(|&&&v| f(v)).unwrap()
	)
}


//////////////////////////////////////////////////////////////////////////////////////////////////



trait ForSingleIndex {
	fn for_single_index(element: usize) -> Self;
}

struct CumDSUNode<T: Add + Sub + Default + Clone> {
	reference: usize,
	size: usize,
	property: T
}

struct CumulativeDSU<T: Add + Sub + Default + Clone> {
	data: Vec<CumDSUNode<T>>
}

impl<T: Add<Output=T> + Sub<Output=T> + Default + Clone> CumulativeDSU<T> {
	fn new(n: usize) -> CumulativeDSU<T> {
		CumulativeDSU {
			data: (0..n).map(|i| CumDSUNode { property: T::default(), reference: i, size: 1 }).collect()
		}
	}

	fn find(&self, mut index: usize) -> (usize, T) {
		let mut cum = self.data[index].property.clone();
		while self.data[index].reference != index {
			index = self.data[index].reference;
			cum = cum + self.data[index].property.clone();
		}

		(index, cum)
	}

	fn unite(&mut self, left_index: usize, right_index: usize) {
		let left_repr = self.find(left_index).0;
		let right_repr = self.find(right_index).0;

		if left_repr == right_repr { return }

		let (&index_small, &index_big) =
			min_max_by_key(&left_repr, &right_repr, |&index| self.data[index].size);

		unsafe {
			let small_node = (self.data.as_mut_ptr().offset(index_small as isize)).as_mut().unwrap();
			let big_node = (self.data.as_mut_ptr().offset(index_big as isize)).as_mut().unwrap();


			// Plug small tree to the big one:
			small_node.reference = index_big;

			// Update supporting information:
			big_node.size += small_node.size;
			small_node.property = small_node.property.clone() - big_node.property.clone();
		}
	}
}

// impl<T: Combinable + Add + Sub> Index<usize> for CumulativeDSU<T> {
// 	type Output = T;
//
// 	fn index(&self, index: usize) -> &Self::Output {
// 		&self.data[index].property
// 	}
// }



fn main() {
	// let mut input = InputReader::new();
	let mut input = io::BufReader::new(io::stdin()).lines();
	let mut output = OutputWriter::new();


	let dsu_size: usize = input.next().unwrap().unwrap().split_ascii_whitespace().next().unwrap().parse::<usize>().unwrap();

	let mut dsu = CumulativeDSU::<isize>::new(dsu_size);


	'main_loop: for this_line in input {
		let this_line = this_line.unwrap();
		if this_line.split_ascii_whitespace().count() == 0 { continue 'main_loop }

		let mut splitted = this_line.split_ascii_whitespace();
		let query_name = splitted.next().unwrap();
		let mut args = splitted.map(|s| s.parse::<isize>().unwrap());

		match query_name {
			"add" => {
				let index = dsu.find((args.next().unwrap() - 1) as usize).0;

				let value = args.next().unwrap();
				dsu.data[index as usize].property += value;
			},
			"get" => {
				let index = args.next().unwrap() - 1;

				let exp = dsu.find(index as usize).1;
				output.println(format!("{}", exp));
			},
			"join" => {
				let (l, r) = (args.next().unwrap() - 1, args.next().unwrap() - 1);
				dsu.unite(l as usize, r as usize);
			},
			_ => panic!(),
		}
	}

}

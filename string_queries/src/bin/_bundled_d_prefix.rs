pub mod string_queries {
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
pub mod scanner {
use super::*;
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
    () => {
        "InputReader: Reached end of input!"
    };
}
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
	pub fn consume_until<F: Fn(char) -> bool>(&mut self, test: F) {
		loop {
			assert!(self.has_more(), EOF!());
			if test(self.peek()) { return; }
			self.consume();
		}
	}
	pub fn consume_until_inclusive<F: Fn(char) -> bool>(&mut self, test: F) {
		self.consume_until(test);
		self.consume();
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
}
pub use scanner::*;
pub mod hash {
const HASH_BASE: u64 = 163;
pub struct StringHasher {
	prefix_hashes: Vec<u64>, // prefix_hashes[i] = hash(s[0..i)), i.e. hash of the prefix of length i. E.g. prefix_hashes[0] = hash("")
	powers: Vec<u64>, // powers[i] = HASH_BASE^i
}
impl StringHasher {
	pub fn new(s: &str) -> Self {
		let mut prefix_hashes = vec![0; s.len() + 1];
		let mut powers = vec![1; s.len() + 1];
		for i in 1..=s.len() {
			prefix_hashes[i] = prefix_hashes[i - 1] * HASH_BASE + s.as_bytes()[i - 1] as u64;
			powers[i] = powers[i - 1] * HASH_BASE;
		}
		Self { prefix_hashes, powers }
	}
	fn n(&self) -> usize { self.prefix_hashes.len() - 1 }
	pub fn substring_hash(&self, l: usize, r: usize) -> u64 {
		(self.prefix_hashes[r] - self.prefix_hashes[l] * self.powers[r - l]) // this gives hash(s[l..r))
	}
}
#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn test_string_hasher() {
		let s = "abacaba";
		let hasher = StringHasher::new(s);
		assert_eq!(hasher.substring_hash(0, 1), 'a' as u64);
		assert_eq!(hasher.substring_hash(1, 2), 'b' as u64);
		assert_eq!(hasher.substring_hash(2, 3), 'a' as u64);
		assert_eq!(hasher.substring_hash(3, 4), 'c' as u64);
		assert_eq!(hasher.substring_hash(4, 5), 'a' as u64);
		assert_eq!(hasher.substring_hash(5, 6), 'b' as u64);
		assert_eq!(hasher.substring_hash(6, 7), 'a' as u64);
	}
	#[test]
	fn test_string_hasher_equal_substrings() {
		let s = "abacaba";
		let hasher = StringHasher::new(s);
		assert_eq!(hasher.substring_hash(0, 1), hasher.substring_hash(6, 7));
		assert_eq!(hasher.substring_hash(0, 3), hasher.substring_hash(4, 7));
		assert_ne!(hasher.substring_hash(0, 1), hasher.substring_hash(1, 2));
		assert_ne!(hasher.substring_hash(0, 3), hasher.substring_hash(1, 4));
	}
}
}
pub use hash::*;
pub mod string_func {
use std::cmp::min;
pub fn prefix_function(s: &str) -> Vec<usize> {
	let mut pi = vec![0; s.len()];
	let mut k = 0;
	for i in 1..s.len() {
		while k > 0 && s.as_bytes()[k] != s.as_bytes()[i] {
			k = pi[k - 1];
		}
		if s.as_bytes()[k] == s.as_bytes()[i] {
			k += 1;
		}
		pi[i] = k;
	}
	pi
}
pub fn z_function(s: &str) -> Vec<usize> {
	let mut z = vec![0; s.len()];
	let mut l = 0;
	let mut r = 0;
	for i in 1..s.len() {
		if i <= r {
			z[i] = min(r - i + 1, z[i - l]);
		}
		while i + z[i] < s.len() && s.as_bytes()[z[i]] == s.as_bytes()[i + z[i]] {
			z[i] += 1;
		}
		if i + z[i] - 1 > r {
			l = i;
			r = i + z[i] - 1;
		}
	}
	z
}
}
pub use string_func::*;
pub mod corasick {
}
pub use corasick::*;
pub mod suffmass {
}
pub use suffmass::*;
pub fn print_vec<T: Display>(vec: &[T]) {
	println!("{}", vec.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
}
fn minmax<T: Ord>(a: T, b: T) -> (T, T) {
	if a <= b {
		(a, b)
	} else {
		(b, a)
	}
}
}
use string_queries::{
	InputReader,
	print_vec,
	prefix_function
};
fn main() {
	let mut input = InputReader::new();
	let p: String = input.next();
	let t: String = input.next();
	let combo = p.clone() + "#" + &t;
	let prefix = prefix_function(&combo);
	let mut result = Vec::new();
	for i in p.len() + 1..combo.len() {
		if prefix[i] == p.len() {
			result.push(i - p.len() - p.len() + 1);
		}
	}
	println!("{}", result.len());
	print_vec(&result);
}

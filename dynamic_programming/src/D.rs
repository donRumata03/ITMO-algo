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
use std::io::BufRead;
// use itertools::{iproduct, Itertools};
use std::iter::Sum;
use std::fmt::Formatter;


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
//
// macro_rules! iproduct {
//     (@flatten $I:expr,) => { ... };
//     (@flatten $I:expr, $J:expr, $($K:expr,)*) => { ... };
//     ($I:expr) => { ... };
//     ($I:expr, $J:expr) => { ... };
//     ($I:expr, $J:expr, $($K:expr),+) => { ... };
// }

const MOD: u64 = 1_000_000_000;
#[derive(Debug)]
#[derive(Copy, Clone)]
struct ModularInt {
	v: u64,
	modulo: u64
}

impl ModularInt {
	pub fn from(v: u64, modulo: u64) -> Self {
		ModularInt { v: v % modulo, modulo }
	}

	pub fn default() -> Self {
		ModularInt { v: 0, modulo: MOD }
	}
}

impl Display for ModularInt {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.v)
	}
}

impl Add for ModularInt {
	type Output = ModularInt;

	fn add(self, rhs: Self) -> Self::Output {
		assert_eq!(self.modulo, rhs.modulo);
		ModularInt::from(self.v + rhs.v, self.modulo)
	}
}

impl AddAssign for ModularInt {
	fn add_assign(&mut self, rhs: Self) {
		*self = *self + rhs;
	}
}

impl Sum for ModularInt {
	fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
		iter.fold(Self::default(), |l, r| l + r)
	}
}

//////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Eq, PartialEq)]
#[derive(std::fmt::Debug)]
#[derive(Hash)]
struct Point (i64, i64);

fn gen_valid_positions() -> Vec<Point> {
	let mut candidates = Vec::new();
	for i in 0i64..4 {
		for j in 0i64..3 {
			candidates.push((i, j));
		}
	}

	candidates.into_iter()
		.map(|(y, x)| Point(y, x))
		.filter(|p| pos_is_valid(p))
		.collect()
}

fn pos_is_valid(pos: &Point) -> bool {
	*pos == Point(3, 1)
		|| [pos.0, pos.1].iter().all(|&c| 0 <= c && c < 3)
}

fn is_knight_move(start: &Point, end: &Point) -> bool {
	// dbg!(start);
	// dbg!(end);
	// dbg!(
	let mut diff = [(start.0 - end.0).abs(), (start.1 - end.1).abs()];
	diff.sort_unstable();

	diff == [1, 2]
}

fn get_knight_moves(start: &Point) -> Vec<Point> {
	// iproduct!( 0i64..4, 0i64..3 )
	gen_valid_positions().into_iter()
		.filter(|p| is_knight_move(start, p))
		.collect()
}

fn main() {
	let mut input = InputReader::new();
	let mut output = OutputWriter::new();

	assert_eq!(
		get_knight_moves(&Point(1, 2))
			.into_iter().collect::<HashSet<_>>(),

		vec![Point(0, 0), Point(3, 1), Point(2, 0)]
			.into_iter().collect::<HashSet<_>>()
	);

	let n = input.next();
	let mut dp = vec![vec![ModularInt::from(1u64, MOD); 3]; 4];
	dp[2][1] = ModularInt::from(0u64, MOD);
	dp[3][1] = ModularInt::from(0u64, MOD);

	for i in 1_usize..n {
		let mut new_dp = vec![vec![ModularInt::from(0u64, MOD); 3]; 4];
		for from in gen_valid_positions() {
			for to in get_knight_moves(&from) {
				new_dp[to.0 as usize][to.1 as usize] += dp[from.0 as usize][from.1 as usize];
			}
		}
		std::mem::swap(&mut dp, &mut new_dp);
	}

	// dbg!(&dp);

	println!("{}", dp.into_iter().map(|vec| vec.into_iter().sum::<ModularInt>()).sum::<ModularInt>())
}
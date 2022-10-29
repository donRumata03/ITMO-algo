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


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VisitColor {
	White,
	Gray,
	Black,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Edge {
	to: usize,
	edge_index: usize,
}

// Graph as an adjacency list
#[derive(Debug, Clone)]
pub struct Graph {
	edges: Vec<Vec<Edge>>,
	total_edges: usize,
}

impl Graph {
	pub fn new(n: usize) -> Self {
		Graph {
			edges: vec![Vec::new(); n],
			total_edges: 0,
		}
	}

	pub fn add_indexed_directed_edge(&mut self, from: usize, to: usize, edge_index: usize) {
		self.edges[from].push(Edge { to, edge_index });
	}

	pub fn add_undirected_edge(&mut self, from: usize, to: usize) {
		let edge_index = self.total_edges;
		self.add_indexed_directed_edge(from, to, edge_index);
		self.add_indexed_directed_edge(to, from, edge_index);
		self.total_edges += 1;
	}

	pub fn add_directed_edge(&mut self, from: usize, to: usize) {
		let edge_index = self.total_edges;
		self.add_indexed_directed_edge(from, to, edge_index);
		self.total_edges += 1;
	}

	pub fn vertexes(&self) -> usize {
		self.edges.len()
	}

	pub fn edges(&self) -> usize {
		self.total_edges
	}
}

#[derive(Debug, Clone)]
pub struct DFSSpace {
	pub time: usize,
	pub visit_colors: Vec<VisitColor>,
	pub t_in: Vec<usize>,
	pub t_out: Vec<usize>,
}

impl DFSSpace {
	pub fn new(graph: &Graph) -> Self {
		let n = graph.vertexes();
		DFSSpace {
			time: 0,
			visit_colors: vec![VisitColor::White; n],
			t_in: vec![0; n],
			t_out: vec![0; n],
		}
	}

	pub fn topological_sort(&mut self, graph: &Graph) -> Vec<usize> {
		let mut order = Vec::new();
		for v in 0..graph.vertexes() {
			if self.visit_colors[v] == VisitColor::White {
				self.dfs(graph, v, &mut order);
			}
		}
		order
	}

	fn dfs(&mut self, graph: &Graph, v: usize, order: &mut Vec<usize>) {
		self.visit_colors[v] = VisitColor::Gray;
		self.t_in[v] = self.time;
		self.time += 1;
		for edge in &graph.edges[v] {
			let to = edge.to;
			if self.visit_colors[to] == VisitColor::White {
				self.dfs(graph, to, order);
			}
		}
		self.visit_colors[v] = VisitColor::Black;
		self.t_out[v] = self.time;
		self.time += 1;
		order.push(v);
	}
}

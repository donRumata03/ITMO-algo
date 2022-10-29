// Searching bridges in undirected graph

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
enum VisitColor {
	White,
	Gray,
	Black,
}

struct Edge {
	to: usize,
	edge_index: usize,
}

fn main() {
	let mut graph = Vec::new();

	let mut input = InputReader::new();
	// let mut output = OutputWriter::new();

	let n: usize = input.next();
	let m: usize = input.next();

	// Add nodes with id 0..n
	for _ in 0..n {
		graph.push(Vec::new());
	}

	// Read m edges and add them to the graph
	for edge_index in 0..m {
		let u: usize = input.next();
		let v: usize = input.next();
		graph[u - 1].push(Edge { to: v - 1, edge_index });
		graph[v - 1].push(Edge { to: u - 1, edge_index });
	}

	// Find bridges
	// Bridge is an edge that if removed, the graph will be disconnected
	// In dfs, if in the subtree of a node, there are edges that point to a node that is higher in the dfs tree,
	// then the edge is not a bridge

	let mut bridges = Vec::new(); // Indexes of edges that are bridges
	let mut t_in = vec![0; n]; // First visit order of nodes
	let mut visit_color = vec![VisitColor::White; n]; // Traversal color of nodes
	let mut time = 0; // Counter for visit order
	let mut highest_reachable = vec![usize::MAX; n]; // Smallest t_in of node to which there is an edge from the subtree of the node

	for node in 0..n {
		if visit_color[node] == VisitColor::White {
			dfs(node, &graph, &mut t_in, &mut visit_color, &mut time, &mut bridges, &mut highest_reachable, None);
		}
	}

	// Debug all
	// dbg!(bridges);
	// dbg!(t_in);
	// dbg!(visit_color);
	// dbg!(highest_reachable);

	// Print all bridges
	bridges.sort();
	println!("{}", bridges.len());
	for bridge in bridges {
		println!("{}", bridge + 1);
	}
}

fn dfs(node: usize, graph: &Vec<Vec<Edge>>, t_in: &mut Vec<usize>, visit_color: &mut Vec<VisitColor>, time: &mut usize, bridges: &mut Vec<usize>, highest_reachable: &mut Vec<usize>, parent: Option<usize>) {
	visit_color[node] = VisitColor::Gray;
	t_in[node] = *time;
	highest_reachable[node] = *time;
	*time += 1;


	for edge in &graph[node] {
		if visit_color[edge.to] == VisitColor::White {
			dfs(edge.to, graph, t_in, visit_color, time, bridges, highest_reachable, Some(node));
			highest_reachable[node] = min(highest_reachable[node], highest_reachable[edge.to]);
		} else if visit_color[edge.to] == VisitColor::Gray {
			// Handle parent separately
			// upper edge from node itself
			if parent != Some(edge.to) {
				highest_reachable[node] = min(highest_reachable[node], t_in[edge.to]);
			}
		}
	}

	// If the node is not the root of the dfs tree and in the subtree there is an edge to a node that is higher in the dfs tree,
	// then the edge is not a bridge
	match parent {
		Some(parent) if highest_reachable[node] == t_in[node] => { // Get index of the edge from the parent to the node
			let edge_index = graph[parent.unwrap()].iter().filter(|&edge| edge.to == node).nth(0);
			if edge_index.is_none() { return; }
			bridges.push(edge_index.unwrap().edge_index);
		}, _ => {}
	}

	visit_color[node] = VisitColor::Black;
}

/*

6 7
1 2
2 3
3 4
1 3
4 5
4 6
5 6

——————————————

3 3
1 2
2 3
3 1

——————————————

4 4
1 2
2 3
3 1
4 1
——————————————

3 2
1 2
2 3

 */

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

pub mod scanner;
pub use scanner::*;

pub mod graph;
pub use graph::*;

pub mod dfs;
pub use dfs::*;

pub mod sat;
pub use sat::*;

pub mod dsu;
pub use dsu::*;

pub mod mst;
pub use mst::*;

pub fn print_vec<T: Display>(vec: &Vec<T>) {
	println!("{}", vec.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
}

fn group_by<T: Clone, K, F>(vec: &Vec<T>, key: F) -> HashMap<K, Vec<T>>
	where F: Fn(&T) -> K, K: Eq + std::hash::Hash
{
	let mut result: HashMap<K, Vec<T>> = HashMap::new();
	for item in vec {
		let key = key(item);
		match result.entry(key) {
			Entry::Occupied(mut entry) => {
				entry.get_mut().push(item.clone());
			}
			Entry::Vacant(entry) => {
				entry.insert(vec![item.clone()]);
			}
		}
	}
	result
}

fn minmax<T: Ord>(a: T, b: T) -> (T, T) {
	if a <= b {
		(a, b)
	} else {
		(b, a)
	}
}


#[derive(Debug, Clone)]
pub struct Decomposition {
	pub elements: usize,
	pub component_list: Vec<Vec<usize>>,
	pub component_map: Vec<usize>,
}

impl Decomposition {
	pub fn from_component_list(component_list: Vec<Vec<usize>>) -> Self {
		let elements = component_list.iter().map(|x| x.len()).sum();

		debug_assert_eq!(elements, component_list.iter().flatten().max().map(|x| x + 1).unwrap_or_default(),
		                 "Components should be indexed from 0 to n - 1 without gaps");

		debug_assert_eq!(elements, HashSet::<usize>::from_iter(
			component_list.iter()
				.flatten()
				.cloned()
		).len(), "There are duplicate vertexes in component list");

		let mut component_map = vec![0; elements];
		for (i, component) in component_list.iter().enumerate() {
			for v in component {
				component_map[*v] = i;
			}
		}
		Decomposition {
			elements,
			component_list,
			component_map,
		}
	}

	pub fn from_component_map(component_map: Vec<usize>) -> Self {
		let elements = component_map.len();
		let component_number = component_map.iter().max().map(|&n| n + 1).unwrap_or_default();

		let mut component_list = vec![Vec::new(); component_number];
		for (i, component) in component_map.iter().enumerate() {
			component_list[*component].push(i);
		}

		// Debug assert that all indexes from 0 to n - 1 are present using array
		debug_assert!(component_list.iter().all(|x| !x.is_empty()),
		              "All indexes from 0 to n - 1 should be present in component map");

		Decomposition {
			elements,
			component_list,
			component_map,
		}
	}

	pub fn component_of(&self, vertex_index: usize) -> usize {
		return self.component_map[vertex_index];
	}
}


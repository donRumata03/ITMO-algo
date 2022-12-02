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

pub mod hash;
pub use hash::*;

pub mod string_func;
pub use string_func::*;

pub mod corasick;
pub use corasick::*;

pub mod suffmass;
pub use suffmass::*;


pub fn print_vec<T: Display>(vec: &Vec<T>) {
	println!("{}", vec.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
}

fn minmax<T: Ord>(a: T, b: T) -> (T, T) {
	if a <= b {
		(a, b)
	} else {
		(b, a)
	}
}



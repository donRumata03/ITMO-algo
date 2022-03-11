use std::ops::{Add, Range};
use std::cmp::min;
use std::marker::PhantomData;

mod operations;
pub use operations::*;

mod engine;
pub use engine::*;

mod mass_read_segment_tree;
pub use mass_read_segment_tree::*;

mod mass_read_write_segment_tree;
pub use mass_read_write_segment_tree::*;

pub mod common_operations;
pub use self::common_operations::*;

pub mod testing_queries;
pub use self::testing_queries::*;

pub mod mass_write_segment_tree;
pub use self::mass_write_segment_tree::*;


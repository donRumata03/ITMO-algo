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

// TODO: implement models with just plain operations for testing

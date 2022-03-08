use std::ops::{Add, Range};
use std::cmp;
use std::cmp::min;
use std::marker::PhantomData;

mod segment_tree;

// use self::segment_tree::*;

// trait ModificationQuery<RE> {}
// trait ReductionQuery<RE> {}




///_____________________________________________________________________________


/// _____________________________________________________________________________


/// _____________________________________________________________________________



/// _____________________________________________________________________________




/// _____________________________________________________________________________



fn main() {
    println!("Hello!");

    let tree =
        segment_tree::MassReadSegmentTree::<
            i64,
            crate::segment_tree::AssignmentModification<i64>,
            crate::segment_tree::SumReduction<i64>
        >::build(vec![1, 2, 3]);

    assert_eq!(tree.data, vec![6, 3, 3, 1, 2, 3, 0]);
}


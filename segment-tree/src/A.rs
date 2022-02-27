use std::ops::{Add, Range};
use std::cmp;
use std::marker::PhantomData;


trait ReductionOp<T> {
	fn neutral() -> T;
	fn apply(left: T, right: T) -> T;
}

trait CommutativeOp<T>: ReductionOp<T> {}
trait DistributiveRelativeTo<T>: ReductionOp<T> {}


struct SumReduction<T: From<i64> + Add<Output=T>> { _p: PhantomData<T> }

impl<T: From<i64> + Add<Output=T>> ReductionOp<T> for SumReduction<T> {
	fn neutral() -> T {
		1.into()
	}

	fn apply(left: T, right: T) -> T {
		left + right
	}
}

///_____________________________________________________________________________

trait ModificationQuery {}
trait ReductionQuery {}

struct ElementModificationQuery<Element> {
	position: usize,
	new_element: Element,
}

impl ModificationQuery for ElementModificationQuery<Element> {
	
}

struct SegmentModificationQuery<Element> {
	segment: Range<usize>,
	new_element: Element,
}

struct ElementReductionQuery<Element> {
	element: usize
}

struct SegmentReductionQuery<Element> {
	segment: Range<usize>
}


trait SegmentTree<ModificationQuery> {

}

///_____________________________________________________________________________

struct SegmentTreeEngine<Element> {
	data: Vec<Element>
}

impl<Element> SegmentTreeEngine<Element> {
	
}


/// _____________________________________________________________________________


fn main() {

}


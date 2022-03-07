use std::ops::{Add, Range};
use std::cmp;
use std::cmp::min;
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

trait ModificationQuery<RElement> {}
trait ReductionQuery<RElement> {}

struct ElementModificationQuery<RElement> {
	position: usize,
	new_element: RElement,
}

// impl<RElement> ModificationQuery for ElementModificationQuery<RElement> {}

struct SegmentModificationQuery<RElement> {
	segment: Range<usize>,
	new_element: RElement,
}


struct ElementReductionQuery<RElement> {
	element_index: usize,
	_t: std::marker::PhantomData<RElement>
}

struct SegmentReductionQuery<RElement> {
	segment: Range<usize>,
	_t: std::marker::PhantomData<RElement>
}


trait SegmentTree<RElement, MQ: ModificationQuery<RElement>, RQ: ReductionQuery<RElement>> {

}

///_____________________________________________________________________________

struct SegmentTreeEngine<RElement> {
	data: Vec<RElement>
}

impl<RElement> SegmentTreeEngine<RElement> {

}

/// _____________________________________________________________________________

struct MassReadSegmentTree<RElement, RQ: ReductionOp<RElement>> {
	e: SegmentTreeEngine<RElement>,
	_t: PhantomData<RQ>
}

impl<RElement, RQ: ReductionOp<RElement>> Default for MassReadSegmentTree<RElement, RQ> {
	fn default() -> Self {
		MassReadSegmentTree {
			e: SegmentTreeEngine{data:Vec::new()},
			_t: Default::default()
		}
	}
}

impl<RElement, RQ: ReductionOp<RElement>> MassReadSegmentTree<RElement, RQ> {
	fn fill_neutral(n: usize) -> Self {
		todo!()
	}

	fn build(initial_data: Vec<usize>) -> Self {
		todo!()
	}

	fn modify_element(q: ElementReductionQuery<RElement>) {
		todo!()
	}

	fn reduce(q: SegmentReductionQuery<RElement>) -> RElement {
		todo!()
	}
}

/// _____________________________________________________________________________

trait ComposableModificationQuery<RElement>: ModificationQuery<RElement> {
	fn compose(old: &Self, new: &Self) -> Self;
}


trait RecountableAfterMassApplication<RElement, MQ: ModificationQuery<RElement>> {
	fn recount(reduction_element: &mut RElement, query: MQ, len: usize);
}

enum SegmentAdditionAssignment {
	Addition(i64),
	Assignment(i64)
}

impl ModificationQuery<i64> for SegmentAdditionAssignment {}

impl ComposableModificationQuery<i64> for SegmentAdditionAssignment {
	fn compose(old: &Self, new: &Self) -> Self {
		match (old, new) {
			(_, SegmentAdditionAssignment::Assignment(v)) => {
				SegmentAdditionAssignment::Assignment(*v)
			},
			(Self::Addition(v_old), Self::Addition(v_new)) => {
				Self::Addition(v_old + v_new)
			},
			(Self::Assignment(v_assigned), Self::Addition(v_added)) => {
				Self::Assignment(v_assigned + v_added)
			}
		}
	}
}

struct MinReduction {

}

impl ReductionOp<i64> for MinReduction {
	fn neutral() -> i64 {
		i64::MAX
	}

	fn apply(left: i64, right: i64) -> i64 {
		min(left, right)
	}
}

impl RecountableAfterMassApplication<i64, SegmentAdditionAssignment> for MinReduction {
	fn recount(reduction_element: &mut i64, query: SegmentAdditionAssignment, _len: usize) {
		match query {
			SegmentAdditionAssignment::Addition(v) => {
				*reduction_element += v;
			},
			SegmentAdditionAssignment::Assignment(v) => {
				*reduction_element = v;
			}
		}
	}
}

/// _____________________________________________________________________________



fn main() {

}


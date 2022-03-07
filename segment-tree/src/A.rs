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


struct SumReduction<T: From<i64> + Add<Output=T>> {  }

impl<T: From<i64> + Add<Output=T>> ReductionOp<T> for SumReduction<T> {
	fn neutral() -> T {
		1.into()
	}

	fn apply(left: T, right: T) -> T {
		left + right
	}
}

///_____________________________________________________________________________

trait ModificationDescriptor<RElement> {
	fn apply(&self, argument: RElement) -> RElement;
}


// trait ModificationQuery<RElement> {}
// trait ReductionQuery<RElement> {}

struct ElementModificationQuery<RElement, MD: ModificationDescriptor<RElement>> {
	position: usize,
	mqd: MD,
}

// impl<RElement> ModificationQuery for ElementModificationQuery<RElement> {}

struct SegmentModificationQuery<RElement, MD: ModificationDescriptor<RElement>> {
	segment: Range<usize>,
	mqd: MD,
}


struct ElementReductionQuery<RElement, RO: ReductionOp<RElement>> {
	element_index: usize,
}

struct SegmentReductionQuery<RElement, RO: ReductionOp<RElement>> {
	segment: Range<usize>
}


trait ElementModifier<RElement, MD: ModificationDescriptor<RElement>> {
	fn modify_element(&mut self, q: ElementModificationQuery<RElement, MD>);
}

trait SegmentModifier<RElement, MD: ModificationDescriptor<RElement>> {
	fn modify_segment(&mut self, q: SegmentModificationQuery<RElement, MD>);
}

impl<
	RElement,
	MD: ModificationDescriptor<RElement>,
	Answerer: SegmentModifier<RElement, MD>
> ElementModifier<RElement, MD> for Answerer {
	fn modify_element(&mut self, q: ElementModificationQuery<RElement, MD>) {
		self.modify_segment(SegmentModificationQuery::<RElement, MD>{
			segment: q.position..q.position + 1,
			mqd: q.mqd,
		})
	}
}

trait ElementReducer<RElement, RO: ReductionOp<RElement>> {
	fn reduce_element(&mut self, q: ElementReductionQuery<RElement, RO>);
}

trait SegmentReducer<RElement, RO: ReductionOp<RElement>> {
	fn reduce_segment(&mut self, q: SegmentReductionQuery<RElement, RO>);
}


impl<
	RElement,
	RO: ReductionOp<RElement>,
	Answerer: SegmentReducer<RElement, RO>
> ElementReducer<RElement, RO> for Answerer {

	fn reduce_element(&mut self, q: ElementReductionQuery<RElement, RO>) {
		self.modify_segment(SegmentReductionQuery::<RElement, RO>{
			segment: q.position..q.position + 1,
		})
	}
}


///_____________________________________________________________________________

struct SegmentTreeEngine<RElement> {
	data: Vec<RElement>
}

impl<RElement> SegmentTreeEngine<RElement> {

}

/// _____________________________________________________________________________

struct MassReadSegmentTree<
	RElement,
	MD: ModificationDescriptor<RElement>,
	RO: ReductionOp<RElement>
> {
	e: SegmentTreeEngine<RElement>,
}

impl<
	RElement,
	MD: ModificationDescriptor<RElement>,
	RO: ReductionOp<RElement>
> Default for MassReadSegmentTree<RElement, MD, RO> {
	fn default() -> Self {
		MassReadSegmentTree {
			e: SegmentTreeEngine{data:Vec::new()},
		}
	}
}

impl<
	RElement,
	MD: ModificationDescriptor<RElement>,
	RO: ReductionOp<RElement>
> MassReadSegmentTree<RElement, MD, RO> {
	fn fill_neutral(n: usize) -> Self {
		todo!()
	}

	fn build(initial_data: Vec<usize>) -> Self {
		todo!()
	}
}

impl<
	RElement,
	MD: ModificationDescriptor<RElement>,
	RO: ReductionOp<RElement>
> ElementModifier<RElement, MD> for MassReadSegmentTree<RElement, MD, RO> {
	fn modify_element(&mut self, q: ElementModificationQuery<RElement, MD>) {
		todo!()
	}
}

impl<
	RElement,
	MD: ModificationDescriptor<RElement>,
	RO: ReductionOp<RElement>
> SegmentReducer<RElement, RO> for MassReadSegmentTree<RElement, MD, RO> {
	fn reduce_segment(&mut self, q: SegmentReductionQuery<RElement, MD>) {
		todo!()
	}
}

/// _____________________________________________________________________________

trait ComposableModificationDescriptor<RElement>: ModificationDescriptor<RElement> {
	fn compose(old: &Self, new: &Self) -> Self;
}


trait RecountableAfterMassApplication<RElement, MQ: ModificationQuery<RElement>> {
	fn recount(reduction_element: &mut RElement, query: MQ, len: usize);
}

enum SegmentAdditionAssignment {
	Addition(i64),
	Assignment(i64)
}

impl ModificationDescriptor<i64> for SegmentAdditionAssignment {
	fn apply(&self, argument: i64) -> i64 {
		match self {
			Self::Addition(&v) => argument + v,
			Self::Assignment(&v) => v,
		}
	}
}

impl ComposableModificationDescriptor<i64> for SegmentAdditionAssignment {
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


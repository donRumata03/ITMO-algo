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

trait ModificationDescriptor<RE> {
	fn apply(&self, argument: RE) -> RE;
}


// trait ModificationQuery<RE> {}
// trait ReductionQuery<RE> {}

struct ElementModificationQuery<RE, MD: ModificationDescriptor<RE>> {
	position: usize,
	mqd: MD,
}

// impl<RE> ModificationQuery for ElementModificationQuery<RE> {}

struct SegmentModificationQuery<RE, MD: ModificationDescriptor<RE>> {
	segment: Range<usize>,
	mqd: MD,
}


struct ElementReductionQuery<RE, RO: ReductionOp<RE>> {
	element_index: usize,
}

struct SegmentReductionQuery<RE, RO: ReductionOp<RE>> {
	segment: Range<usize>
}


trait ElementModifier<RE, MD: ModificationDescriptor<RE>> {
	fn modify_element(&mut self, q: ElementModificationQuery<RE, MD>);
}

trait SegmentModifier<RE, MD: ModificationDescriptor<RE>> {
	fn modify_segment(&mut self, q: SegmentModificationQuery<RE, MD>);
}

// TODO: have front-end and back-end traits: every type only implements itself one of
// TODO: Segment or query, but for frontend segment one also implements element one
// impl<
// 	RE,
// 	MD: ModificationDescriptor<RE>,
// 	Answerer: SegmentModifier<RE, MD>
// > ElementModifier<RE, MD> for Answerer {
// 	fn modify_element(&mut self, q: ElementModificationQuery<RE, MD>) {
// 		self.modify_segment(SegmentModificationQuery::<RE, MD>{
// 			segment: q.position..q.position + 1,
// 			mqd: q.mqd,
// 		})
// 	}
// }

trait ElementReducer<RE, RO: ReductionOp<RE>> {
	fn reduce_element(&mut self, q: ElementReductionQuery<RE, RO>);
}

trait SegmentReducer<RE, RO: ReductionOp<RE>> {
	fn reduce_segment(&mut self, q: SegmentReductionQuery<RE, RO>);
}


// impl<
// 	RE,
// 	RO: ReductionOp<RE>,
// 	Answerer: SegmentReducer<RE, RO>
// > ElementReducer<RE, RO> for Answerer {
//
// 	fn reduce_element(&mut self, q: ElementReductionQuery<RE, RO>) {
// 		self.modify_segment(SegmentReductionQuery::<RE, RO>{
// 			segment: q.position..q.position + 1,
// 		})
// 	}
// }


///_____________________________________________________________________________

struct SegmentTreeEngine<RE, RO: ReductionOp<RE>> {
	data: Vec<RE>
}

impl<RE, RO: ReductionOp<RE>> SegmentTreeEngine<RE, RO> {
	fn left_child(i: usize) -> usize {
		2 * i + 1
	}

	fn right_child(i: usize) -> usize {
		2 * i + 2
	}

	fn parent(i: usize) -> usize {
		(i - 1) / 2
	}

	fn smallest_pow_of_two_size(n: usize) -> usize {
		msb(n)
			.map_or(0, |b| {
				if 2usize.pow(b as u32) == n {
					b
				} else { b + 1 }
			})
	}

	fn fill_neutral(n: usize) -> Self {
		todo!()
	}

	fn decompose_into_segments() -> Vec<Range<usize>> {
		todo!()
	}

	fn visit() {

	}
}

/// _____________________________________________________________________________

fn msb(value: usize) -> Option<usize> {
	let z = value.leading_zeros();

	if z == 64 { None }
	else { Some(63 - z as usize) }
}

/// _____________________________________________________________________________

struct MassReadSegmentTree<
	RE,
	MD: ModificationDescriptor<RE>,
	RO: ReductionOp<RE>
> {
	data: Vec<RE>,
}

impl<
	RE,
	MD: ModificationDescriptor<RE>,
	RO: ReductionOp<RE>
> Default for MassReadSegmentTree<RE, MD, RO> {
	fn default() -> Self {
		MassReadSegmentTree {
			data: Vec::new(),
		}
	}
}

impl<
	RE,
	MD: ModificationDescriptor<RE>,
	RO: ReductionOp<RE>
> MassReadSegmentTree<RE, MD, RO> {
	fn fill_neutral(n: usize) -> Self {
		MassReadSegmentTree {
			data: vec![RO::neutral(); SegmentTreeEngine::smallest_pow_of_two_size(n)]
		}
	}

	fn build(initial_data: Vec<usize>) -> Self {
		todo!()
	}
}

impl<
	RE,
	MD: ModificationDescriptor<RE>,
	RO: ReductionOp<RE>
> ElementModifier<RE, MD> for MassReadSegmentTree<RE, MD, RO> {
	fn modify_element(&mut self, q: ElementModificationQuery<RE, MD>) {
		todo!()
	}
}

impl<
	RE,
	MD: ModificationDescriptor<RE>,
	RO: ReductionOp<RE>
> SegmentReducer<RE, RO> for MassReadSegmentTree<RE, MD, RO> {
	fn reduce_segment(&mut self, q: SegmentReductionQuery<RE, RO>) {
		todo!()
	}
}

/// _____________________________________________________________________________

trait ComposableModificationDescriptor<RE>: ModificationDescriptor<RE> {
	fn compose(old: &Self, new: &Self) -> Self;
}


trait RecountableAfterMassApplication<RE, MD: ModificationDescriptor<RE>> {
	fn recount(reduction_element: &mut RE, query: MD, len: usize);
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

struct MassReadWriteSegmentTree<
	RE,
	MD: ComposableModificationDescriptor<RE>,
	RO: ReductionOp<RE>
> where RO: RecountableAfterMassApplication<RE, MD> {
	e: SegmentTreeEngine<RE>,
}

impl<
	RE,
	MD: ComposableModificationDescriptor<RE>,
	RO: ReductionOp<RE>
> SegmentModifier<RE, MD> for MassReadWriteSegmentTree<RE, MD, RO>
	where RO: RecountableAfterMassApplication<RE, MD>
{
	fn modify_segment(&mut self, q: ElementModificationQuery<RE, MD>) {
		todo!()
	}
}

impl<
	RE,
	MD: ComposableModificationDescriptor<RE>,
	RO: ReductionOp<RE>
> SegmentReducer<RE, RO> for MassReadWriteSegmentTree<RE, MD, RO>
	where RO: RecountableAfterMassApplication<RE, MD>
{
	fn reduce_segment(&mut self, q: SegmentReductionQuery<RE, RO>) {
		todo!()
	}
}



/// _____________________________________________________________________________



fn main() {

}


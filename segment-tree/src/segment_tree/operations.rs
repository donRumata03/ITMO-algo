use super::*;


pub trait ReductionOp<T> {
    fn neutral() -> T;
    fn apply(left: T, right: T) -> T;
}

pub trait CommutativeOp<T>: ReductionOp<T> {}
pub trait DistributiveRelativeTo<T>: ReductionOp<T> {}



///_____________________________________________________________________________

/// Type that represents set of possible modifications of elements
pub trait ModificationDescriptor<RE> {
    fn apply(&self, argument: RE) -> RE;
}

/// Different kinds of queries { modification, reduction } × { segment, element }
pub struct ElementModificationQuery<RE, MD: ModificationDescriptor<RE>> {
    position: usize,
    mqd: MD,
}

// impl<RE> ModificationQuery for ElementModificationQuery<RE> {}

pub struct SegmentModificationQuery<RE, MD: ModificationDescriptor<RE>> {
    segment: Range<usize>,
    mqd: MD,
}


pub struct ElementReductionQuery<RE, RO: ReductionOp<RE>> {
    element_index: usize,
}

pub struct SegmentReductionQuery<RE, RO: ReductionOp<RE>> {
    segment: Range<usize>
}

/// Traits for a data structure answering different type of queries
pub trait ElementModifier<RE, MD: ModificationDescriptor<RE>> {
    fn modify_element(&mut self, q: ElementModificationQuery<RE, MD>);
}

pub trait SegmentModifier<RE, MD: ModificationDescriptor<RE>> {
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

pub trait ElementReducer<RE, RO: ReductionOp<RE>> {
    fn reduce_element(&mut self, q: ElementReductionQuery<RE, RO>);
}

pub trait SegmentReducer<RE, RO: ReductionOp<RE>> {
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


/// Composition and recounting (for both reduction and modification on segment)

pub trait ComposableModificationDescriptor<RE>: ModificationDescriptor<RE> {
    fn compose(old: &Self, new: &Self) -> Self;
}


pub trait RecountableAfterMassApplication<RE, MD: ModificationDescriptor<RE>> {
    fn recount(reduction_element: &mut RE, query: MD, len: usize);
}

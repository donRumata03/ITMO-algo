use super::*;

pub trait ReductionElement: Clone {

}

pub trait ReductionOp<T: ReductionElement> {
    fn neutral() -> T;
    fn apply(left: T, right: T) -> T;
}

pub trait CommutativeOp<T: ReductionElement>: ReductionOp<T> {}
pub trait DistributiveRelativeTo<T: ReductionElement>: ReductionOp<T> {}



///_____________________________________________________________________________

/// Type that represents set of possible modifications of elements
pub trait ModificationDescriptor<RE: ReductionElement> {
    fn apply(&self, argument: RE) -> RE;
}

/// Different kinds of queries { modification, reduction } × { segment, element }
pub struct ElementModificationQuery<RE: ReductionElement, MD: ModificationDescriptor<RE>> {
    pub position: usize,
    pub mqd: MD,
    pub(crate) _re: PhantomData<RE>
}

// impl<RE> ModificationQuery for ElementModificationQuery<RE> {}

pub struct SegmentModificationQuery<RE: ReductionElement, MD: ModificationDescriptor<RE>> {
    pub segment: Range<usize>,
    pub mqd: MD,
    _re: PhantomData<RE>
}


pub struct ElementReductionQuery<RE: ReductionElement, RO: ReductionOp<RE>> {
    pub element_index: usize,
    _re: PhantomData<RE>,
    _ro: PhantomData<RO>
}

pub struct SegmentReductionQuery<RE: ReductionElement, RO: ReductionOp<RE>> {
    pub segment: Range<usize>,
    pub(crate) _re: PhantomData<RE>,
    pub(crate) _ro: PhantomData<RO>
}

/// Traits for a data structure answering different type of queries
pub trait ElementModifier<RE: ReductionElement, MD: ModificationDescriptor<RE>> {
    fn modify_element(&mut self, q: ElementModificationQuery<RE, MD>);
}

pub trait SegmentModifier<RE: ReductionElement, MD: ModificationDescriptor<RE>> {
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

pub trait ElementReducer<RE: ReductionElement, RO: ReductionOp<RE>> {
    fn reduce_element(&mut self, q: ElementReductionQuery<RE, RO>);
}

pub trait SegmentReducer<RE: ReductionElement, RO: ReductionOp<RE>> {
    fn reduce_segment(&mut self, q: &SegmentReductionQuery<RE, RO>) -> RE;
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

pub trait ComposableModificationDescriptor<RE: ReductionElement>: ModificationDescriptor<RE> {
    fn compose(old: &Self, new: &Self) -> Self;
}


pub trait RecountableAfterMassApplication<RE: ReductionElement, MD: ModificationDescriptor<RE>> {
    fn recount(reduction_element: &mut RE, query: MD, len: usize);
}

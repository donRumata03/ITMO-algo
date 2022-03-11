/// Module for universal testing of reducing and modifying query answerers
/// Some traits capable for indicating ability to answer certain queries should be implemented
/// by tested implementation.
///
/// The traits are \[Segment|Element\]\[Modifier|Reducer\])
/// As the reference implementation `DummyQueryAnswerer` is taken.
/// It just dummily maintains the whole array and computes the reduction


use std::marker::PhantomData;
use super::operations::*;


struct DummyQueryAnswerer<
	RE: ReductionElement,
	MD: ModificationDescriptor<RE>,
	RO: ReductionOp<RE>
> {
	data: Vec<RE>,
	_m: PhantomData<MD>,
	_r: PhantomData<RO>
}

impl<
	RE: ReductionElement,
	MD: ModificationDescriptor<RE>,
	RO: ReductionOp<RE>
> SegmentReducer<RE, RO> for DummyQueryAnswerer<RE, MD, RO> {
	fn reduce_segment(&mut self, q: &SegmentReductionQuery<RE, RO>) -> RE {
		(&self.data[q.segment.clone()]).iter()
			.cloned()
			.fold(RO::neutral(), RO::apply)
	}
}



impl<
	RE: ReductionElement,
	MD: ModificationDescriptor<RE>,
	RO: ReductionOp<RE>
> SegmentModifier<RE, MD> for DummyQueryAnswerer<RE, MD, RO> {
	fn modify_segment(&mut self, q: &SegmentModificationQuery<RE, MD>) {
		self.data[q.segment.clone()].iter_mut()
			.map(|element| *element = q.mqd.apply(element.clone()));
	}
}


impl<
	RE: ReductionElement,
	MD: ModificationDescriptor<RE>,
	RO: ReductionOp<RE>
> ElementReducer<RE, RO> for DummyQueryAnswerer<RE, MD, RO> {
	fn reduce_element(&mut self, q: &ElementReductionQuery<RE, RO>) -> RE {
		self.reduce_segment( &(q.clone().into()))
	}
}



impl<
	RE: ReductionElement,
	MD: ModificationDescriptor<RE>,
	RO: ReductionOp<RE>
> ElementModifier<RE, MD> for DummyQueryAnswerer<RE, MD, RO> {
	fn modify_element(&mut self, q: &ElementModificationQuery<RE, MD>) {
		self.modify_segment(&(q.clone().into()));
	}
}



/// Runs random queries of allowed types in random order and compares it with reference model
/// TODO: decide how to do it generically
pub fn run_query_tests() {
	todo!()
}
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

// impl<
// 	RE: ReductionElement,
// 	MD: ModificationDescriptor<RE>,
// 	RO: ReductionOp<RE>
// > SegmentReducer<RE, RO> for DummyQueryAnswerer<RE, MD, RO> {
//
// }
//
//
//
// impl<
// 	RE: ReductionElement,
// 	MD: ModificationDescriptor<RE>,
// 	RO: ReductionOp<RE>
// > ElementModifier<RE, MD> for MassReadSegmentTree<RE, MD, RO> {
// 	fn modify_element(&mut self, q: ElementModificationQuery<RE, MD>) {
// 		self.modify_element_impl(q);
// 	}
// }

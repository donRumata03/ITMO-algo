use std::fmt::Debug;
use super::*;

pub struct MassWriteSegmentTree<
	RE: ReductionElement,
	MD: ComposableModificationDescriptor<RE>,
	RO: ReductionOp<RE>
> {
	eng: SegmentTreeEngine<RE, RO, MD>,
	_m: PhantomData<MD>,
	_r: PhantomData<RO>
}

impl<
	RE: ReductionElement,
	MD: ComposableModificationDescriptor<RE>,
	RO: ReductionOp<RE>
> MassWriteSegmentTree<RE, MD, RO> {
	pub fn build(initial_data: &Vec<RE>) -> Self {

	}
}

use super::*;


struct MassReadWriteSegmentTree<
    RE,
    MD: ComposableModificationDescriptor<RE>,
    RO: ReductionOp<RE>
> where RO: RecountableAfterMassApplication<RE, MD> {
    e: SegmentTreeEngine<RE, RO>,
    _m: PhantomData<MD>,
    _r: PhantomData<RO>,
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


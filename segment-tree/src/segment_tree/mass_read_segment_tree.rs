use super::*;

pub struct MassReadSegmentTree<
    RE: ReductionElement,
    MD: ModificationDescriptor<RE>,
    RO: ReductionOp<RE>
> {
    pub(crate) data: Vec<RE>,
    _m: PhantomData<MD>,
    _r: PhantomData<RO>,
}

impl<
    RE: ReductionElement,
    MD: ModificationDescriptor<RE>,
    RO: ReductionOp<RE>
> MassReadSegmentTree<RE, MD, RO> {
    fn with_data(data: Vec<RE>) -> MassReadSegmentTree<RE, MD, RO> {
        Self {
            data: data,
            _m: Default::default(),
            _r: Default::default(),
        }
    }

    pub fn fill_neutral(n: usize) -> Self {
        Self::with_data(
            vec![RO::neutral(); SegmentTreeEngine::<RE, RO>::smallest_pow_of_two_size(n)]
        )
    }

    fn reduce_node(&mut self, parent_index: usize) {
        self.data[parent_index] = RO::apply(
            self.data[SegmentTreeEngine::<RE, RO>::left_child(parent_index)].clone(),
            self.data[SegmentTreeEngine::<RE, RO>::right_child(parent_index)].clone()
        );
    }

    fn is_floor_node(&self, node_index: usize) -> bool {
        node_index >= SegmentTreeEngine::<RE, RO>::floor_start(self.data.len())
    }

    fn down_recursive_update_node_reductions(&mut self, root: usize) {
        if !self.is_floor_node(root) {
            println!("{}, ", root);
            self.down_recursive_update_node_reductions(SegmentTreeEngine::<RE, RO>::left_child(root));
            self.down_recursive_update_node_reductions(SegmentTreeEngine::<RE, RO>::right_child(root));

            self.reduce_node(root);
        }
    }

    pub fn build(initial_data: Vec<RE>) -> Self {
        let mut res = Self::fill_neutral(initial_data.len());
        let data_start = SegmentTreeEngine::<RE, RO>::floor_start(res.data.len());

        let to_copy = &mut res.data[data_start..data_start + initial_data.len()];

        to_copy.iter_mut()
            .zip(initial_data.iter())
            .for_each(|(tree_ptr, data_ptr)| *tree_ptr = data_ptr.clone())
        ;

        res.down_recursive_update_node_reductions(0);

        res
    }
}

impl<
    RE: ReductionElement,
    MD: ModificationDescriptor<RE>,
    RO: ReductionOp<RE>
> ElementModifier<RE, MD> for MassReadSegmentTree<RE, MD, RO> {
    fn modify_element(&mut self, q: ElementModificationQuery<RE, MD>) {
        todo!()
    }
}

impl<
    RE: ReductionElement,
    MD: ModificationDescriptor<RE>,
    RO: ReductionOp<RE>
> SegmentReducer<RE, RO> for MassReadSegmentTree<RE, MD, RO> {
    fn reduce_segment(&mut self, q: SegmentReductionQuery<RE, RO>) {
        todo!()
    }
}


#[cfg(test)]
mod tests {
    use crate::segment_tree::mass_read_segment_tree::MassReadSegmentTree;
    use crate::segment_tree::{AssignmentModification, SumReduction};

    #[test]
    fn test_building() {
        let tree =
            MassReadSegmentTree::<
                i64,
                AssignmentModification<i64>,
                SumReduction<i64>
            >::build(vec![1, 2, 3]);

        assert_eq!(tree.data, vec![6, 3, 3, 1, 2, 3, 0]);
    }
}
use std::fmt::Debug;
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

/// Private impls
impl<
    RE: ReductionElement + Debug,
    MD: ModificationDescriptor<RE>,
    RO: ReductionOp<RE>
> MassReadSegmentTree<RE, MD, RO> {
    pub fn fill_neutral(n: usize) -> Self {
        Self::with_data(
            vec![RO::neutral(); SegmentTreeEngine::<RE, RO>::smallest_pow_of_two_size(n)]
        )
    }

    fn with_data(data: Vec<RE>) -> MassReadSegmentTree<RE, MD, RO> {
        Self {
            data: data,
            _m: Default::default(),
            _r: Default::default(),
        }
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


    fn update_node_reductions_down_from(&mut self, root: usize) {
        if !self.is_floor_node(root) {
            self.update_node_reductions_down_from(SegmentTreeEngine::<RE, RO>::left_child(root));
            self.update_node_reductions_down_from(SegmentTreeEngine::<RE, RO>::right_child(root));

            self.reduce_node(root);
        }
    }

    /// All the nodes «depending» on bottom but not bottom itself
    fn update_node_reductions_up_from(&mut self, bottom_tree_index: usize) {
        let parent = SegmentTreeEngine::<RE, RO>::parent(bottom_tree_index);
        if let Some(parent) = parent {
            self.reduce_node(parent);
            self.update_node_reductions_up_from(parent);
        }
    }


    fn modify_element_impl(&mut self, q: ElementModificationQuery<RE, MD>) {
        let tree_index = SegmentTreeEngine::<RE, RO>::initial_element_tree_index(self.data.len(), q.position);
        self.data[tree_index] = q.mqd.apply(self.data[tree_index].clone());

        self.update_node_reductions_up_from(tree_index)
    }

    fn reduce_segment_impl(&mut self, q: SegmentReductionQuery<RE, RO>) {
        todo!()
    }
}


/// Public ad-hoc impls
impl<
    RE: ReductionElement + Debug,
    MD: ModificationDescriptor<RE>,
    RO: ReductionOp<RE>
> MassReadSegmentTree<RE, MD, RO> {
    pub fn build(initial_data: Vec<RE>) -> Self {
        let mut res = Self::fill_neutral(initial_data.len());
        let data_start = SegmentTreeEngine::<RE, RO>::floor_start(res.data.len());

        let to_copy = &mut res.data[data_start..data_start + initial_data.len()];

        to_copy.clone_from_slice(&initial_data);

        res.update_node_reductions_down_from(0);

        res
    }
}

impl<
    RE: ReductionElement + Debug,
    MD: ModificationDescriptor<RE>,
    RO: ReductionOp<RE>
> ElementModifier<RE, MD> for MassReadSegmentTree<RE, MD, RO> {
    fn modify_element(&mut self, q: ElementModificationQuery<RE, MD>) {
        self.modify_element_impl(q);
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
    use crate::segment_tree::{AssignmentModification, MassReadSegmentTree, SumReduction};

    fn build(source: Vec<i64>) -> MassReadSegmentTree::<
        i64,
        AssignmentModification<i64>,
        SumReduction<i64>
    > {
        MassReadSegmentTree::<
            i64,
            AssignmentModification<i64>,
            SumReduction<i64>
        >::build(source)
    }

    mod building_tests {
        use crate::segment_tree::{AssignmentModification, MassReadSegmentTree, SumReduction};
        use crate::segment_tree::mass_read_segment_tree::tests::build;

        fn verify_building(source: Vec<i64>, expected: Vec<i64>) {
            let tree =
                build(source);

            assert_eq!(tree.data, expected);

        }

        #[test]
        fn test_building() {
            verify_building(vec![1, 2, 3], vec![6, 3, 3, 1, 2, 3, 0, 0]);
        }

        #[test]
        fn test_building_two() {
            verify_building(vec![1, 2], vec![3, 1, 2, 0]);
        }

        #[test]
        fn test_building_single() {
            verify_building(vec![1], vec![1, 0]);
        }
    }

    mod modifying_tests {
        use crate::segment_tree::{AssignmentModification, ElementModificationQuery, ElementModifier};
        use crate::segment_tree::mass_read_segment_tree::tests::build;

        #[test]
        fn modifying_test() {
            let mut tree = build(vec![1, 2, 3]);
            tree.modify_element(ElementModificationQuery {
                position: 1,
                mqd: AssignmentModification{assigned_value: 10},
                _re: Default::default()
            });

            assert_eq!(tree.data, build(vec![1, 10, 3]).data);

            tree.modify_element(ElementModificationQuery {
                position: 0,
                mqd: AssignmentModification{assigned_value: 42},
                _re: Default::default()
            });

            tree.modify_element(ElementModificationQuery {
                position: 2,
                mqd: AssignmentModification{assigned_value: -566},
                _re: Default::default()
            });

            assert_eq!(tree.data, build(vec![42, 10, -566]).data);
        }
    }


}


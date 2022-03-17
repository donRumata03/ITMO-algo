use std::convert::identity;
use std::fmt::Debug;
use super::*;

macro_rules! Engine {
    () => {SegmentTreeEngine::<RE, RO, RE>}
}

pub struct MassReadSegmentTree<
    RE: ReductionElement,
    MD: ModificationDescriptor<RE>,
    RO: ReductionOp<RE>
> {
    e: SegmentTreeEngine<RE, RO, RE>,
    _md: MD
}

/// Private impls
impl<
    RE: ReductionElement,
    MD: ModificationDescriptor<RE>,
    RO: ReductionOp<RE>
> MassReadSegmentTree<RE, MD, RO> {
    pub fn fill_neutral(n: usize) -> Self {
        Self {
            e: SegmentTreeEngine::fill_neutral(n),
            _md: Default::default()
        }
    }


    fn reduce_node(&mut self, node: &NodePositionDescriptor) {
        let left_node = self.e.node_left_child(node);
        let right_node = self.e.node_right_child(node);
        self.e[node] = RO::apply(
            self.e[left_node],
            self.e[right_node]
        );
        //
        // self.data[parent_index] = RO::apply(
        //     self.data[SegmentTreeEngine::<RE, RO, RE>::left_child_index(parent_index)].clone(),
        //     self.data[SegmentTreeEngine::<RE, RO, RE>::right_child_index(parent_index)].clone()
        // );
    }

    //
    // fn is_floor_node(&self, node: &NodePositionDescriptor) -> bool {
    //
    // }

    /// Updates reductions in the whole subtree owned by `root` node (including `root` itself)
    /// Floor nodes are taken as granted, other selected nodes are updated «from down to up»
    fn update_node_reductions_down_from(&mut self, root: &NodePositionDescriptor) {
        if !self.e.node_is_floor(root) {
            self.update_node_reductions_down_from(
                &self.e.node_left_child(root).unwrap()
            );
            self.update_node_reductions_down_from(
                &self.e.node_right_child(root).unwrap()
            );

            self.reduce_node(root);
        }
    }

    /// All the nodes «depending» on bottom but not bottom itself
    fn update_node_reductions_up_from(&mut self, bottom_node: &NodePositionDescriptor) {
        let parent = self.e.node_parent(bottom_node);
        if let Some(parent) = parent {
            self.reduce_node(&parent);
            self.update_node_reductions_up_from(&parent);
        }
    }

    /// Change the value at corresponding position on floor
    /// Update the tower depending on it
    fn modify_element_impl(&mut self, q: &ElementModificationQuery<RE, MD>) {
        let leaf_node = self.e.initial_array_node(q.position);
        self.e[&leaf_node] = q.mqd.apply(self.e[&leaf_node].clone());

        self.update_node_reductions_up_from(&leaf_node)
    }

    // /// Returns reduction on q.segment intersected with segment controlled by the vertex
    // fn reduce_segment_impl(
    //     &mut self,
    //     tree_index: usize,
    //     controlled_segment: Range<usize>,
    //     q: &SegmentReductionQuery<RE, RO>
    // ) -> Option<RE> {
    //     // If this node has nothing to do with query, return None
    //     if SegmentTreeEngine::<RE, RO>::intersect_ranges(&controlled_segment, &q.segment).is_empty() {
    //         return None;
    //     }
    //
    //     // If controlled segment is fully in query range, return it full
    //     if SegmentTreeEngine::<RE, RO>::contains_range(&q.segment, &controlled_segment) {
    //         return Some(self.data[tree_index].clone());
    //     }
    //
    //     // Otherwise, glue answer from left and right queries
    //     let children_ranges = SegmentTreeEngine::<RE, RO>::half_split_range(&controlled_segment);
    //     let left_result = self.reduce_segment_impl(
    //         SegmentTreeEngine::<RE, RO>::left_child_index(tree_index),
    //         children_ranges.0,
    //         q
    //     );
    //
    //     let right_result = self.reduce_segment_impl(
    //         SegmentTreeEngine::<RE, RO>::right_child_index(tree_index),
    //         children_ranges.1,
    //         q
    //     );
    //
    //     left_result.iter()
    //         .chain(right_result.iter()).
    //         cloned()
    //         .reduce(RO::apply)
    // }
}


/// Public ad-hoc impls
impl<
    RE: ReductionElement + Debug,
    MD: ModificationDescriptor<RE>,
    RO: ReductionOp<RE>
> MassReadSegmentTree<RE, MD, RO> {

    /// — Assign floor
    ///
    /// — Build up from floor
    pub fn build(initial_data: &Vec<RE>) -> Self {
        let mut res = Self::fill_neutral(initial_data.len());

        res.e.set_floor(initial_data);
        res.e.rebuild_from_floor(RO::apply);

        // let data_start = SegmentTreeEngine::<RE, RO, RE>::floor_start(res.data.len());
        //
        // let to_copy = &mut res.data[data_start..data_start + initial_data.len()];
        //
        // to_copy.clone_from_slice(&initial_data);
        //
        // res.update_node_reductions_down_from(0);

        res
    }
}

impl<
    RE: ReductionElement,
    MD: ModificationDescriptor<RE>,
    RO: ReductionOp<RE>
> ElementModifier<RE, MD> for MassReadSegmentTree<RE, MD, RO> {
    fn modify_element(&mut self, q: &ElementModificationQuery<RE, MD>) {
        self.modify_element_impl(q);
    }
}

impl<
    RE: ReductionElement,
    MD: ModificationDescriptor<RE>,
    RO: ReductionOp<RE>
> SegmentReducer<RE, RO> for MassReadSegmentTree<RE, MD, RO> {
    fn reduce_segment(&mut self, q: &SegmentReductionQuery<RE, RO>) -> RE {
        self.e.reduce(q.segment.clone(), identity, RO::apply)
            .unwrap_or(RO::neutral())

        // self.reduce_segment_impl(
        //     0,
        //     0..SegmentTreeEngine::<RE, RO>::array_size(self.data.len()),
        //     q
        // )
        // .unwrap_or(RO::neutral())
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
        >::build(&source)
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
            //                                               0  1  2  3  4  5  6  7
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
            tree.modify_element(&ElementModificationQuery::new(
                1,
                AssignmentModification{assigned_value: 10}
            ));

            assert_eq!(tree.data, build(vec![1, 10, 3]).data);

            tree.modify_element(&ElementModificationQuery::new(
                0,
                AssignmentModification{assigned_value: 42}
            ));
            tree.modify_element(&ElementModificationQuery::new(
                2,
                AssignmentModification{assigned_value: -566}
            ));
            assert_eq!(tree.data, build(vec![42, 10, -566]).data);
        }
    }

    mod reducing_tests {
        use crate::segment_tree::mass_read_segment_tree::tests::build;
        use crate::segment_tree::{SegmentReducer, SegmentReductionQuery};

        #[test]
        fn test_reducing() {
            let mut tree = build(vec![1, 2, 3]);

            let mut query = |range| tree.reduce_segment(&SegmentReductionQuery::new(range));

            assert_eq!(query(0..0), 0);
            assert_eq!(query(0..1), 1);
            assert_eq!(query(0..2), 3);
            assert_eq!(query(0..3), 6);

            assert_eq!(query(1..1), 0);
            assert_eq!(query(1..2), 2);
            assert_eq!(query(1..3), 5);

            assert_eq!(query(2..2), 0);
            assert_eq!(query(2..3), 3);
        }
    }
}


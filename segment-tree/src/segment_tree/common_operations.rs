use super::*;


#[derive(Clone)]
pub struct SumReduction<T: From<i64> + Add<Output=T>> { _t: PhantomData<T> }

impl<T: From<i64> + Add<Output=T> + ReductionElement> ReductionOp<T> for SumReduction<T> {
    fn neutral() -> T {
        0.into()
    }

    fn apply(left: T, right: T) -> T {
        left + right
    }
}

#[derive(Clone, Debug)]
pub enum AssignmentModification<RE: ReductionElement> {
    AssignedValue(RE),
    Identity
}

impl<RE: ReductionElement> AssignmentModification<RE> {
    pub fn assign(value: RE) -> AssignmentModification<RE> {
        AssignmentModification::AssignedValue(value)
    }
}

impl<RE: ReductionElement> ModificationDescriptor<RE> for AssignmentModification<RE> {
    fn identity() -> Self {
        Self::Identity
    }

    fn apply(&self, argument: RE) -> RE {
        match self {
            AssignmentModification::AssignedValue(v) => {
                v.clone()
            }
            AssignmentModification::Identity => {
                argument
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum SegmentAdditionAssignment {
    Addition(i64),
    Assignment(i64)
}

impl ReductionElement for i64 {}

impl ModificationDescriptor<i64> for SegmentAdditionAssignment {
    fn identity() -> Self {
        Self::Addition(0)
    }

    fn apply(&self, argument: i64) -> i64 {
        match self {
            Self::Addition(v) => argument + *v,
            Self::Assignment(v) => *v,
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

#[derive(Debug, Clone)]
pub struct MinReduction {

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

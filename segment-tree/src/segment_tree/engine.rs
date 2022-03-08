use super::*;

fn msb(value: usize) -> Option<usize> {
    let z = value.leading_zeros();

    if z == 64 { None }
    else { Some(63 - z as usize) }
}


pub struct NodeDescriptor {
    pub tree_index: usize,
    pub l: usize,
    pub r: usize
}



pub struct SegmentTreeEngine<RE: ReductionElement, RO: ReductionOp<RE>> {
    data: Vec<RE>,
    _rp: PhantomData<RO>
}

impl<RE: ReductionElement, RO: ReductionOp<RE>> SegmentTreeEngine<RE, RO> {
    pub(crate) fn left_child(i: usize) -> usize {
        2 * i + 1
    }

    pub(crate) fn right_child(i: usize) -> usize {
        2 * i + 2
    }

    fn parent(i: usize) -> usize {
        (i - 1) / 2
    }

    pub(crate) fn smallest_pow_of_two_size(n: usize) -> usize {
        msb(n)
            .map_or(0, |b| {
                2_usize.pow(if 2usize.pow(b as u32) == n {
                    b
                } else { b + 1 }
             as u32 + 1)})
    }

    pub(crate) fn floor_start(tree_size: usize) -> usize {
        assert!(tree_size.is_power_of_two());

        tree_size / 2
    }

    fn fill_neutral(n: usize) -> Self {
        todo!()
    }

    fn decompose_into_segments() -> Vec<Range<usize>> {
        todo!()
    }

    fn visit() {
        todo!()
    }
}

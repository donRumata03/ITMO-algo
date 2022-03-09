use super::*;

fn msb(value: usize) -> Option<usize> {
	let z = value.leading_zeros();

	if z == 64 { None }
	else { Some(63 - z as usize) }
}


pub struct NodePositionDescriptor {
	pub tree_index: usize,
	pub curated_segment: Range<usize>
}

// impl NodePositionDescriptor {
//
// }

/// May contain only reduction information or may be also pending modification queries, for example.
/// Its content depends on implementation details of particular segment tree
pub struct SegmentTreeNode {

}

pub struct SegmentTreeEngine<RE: ReductionElement, RO: ReductionOp<RE>> {
	data: Vec<RE>,
	_rp: PhantomData<RO>
}

impl<RE: ReductionElement, RO: ReductionOp<RE>> SegmentTreeEngine<RE, RO> {
	fn half_split_range(rng: &Range<usize>) -> (Range<usize>, Range<usize>) {
		let mid = (rng.start + rng.end) / 2;
		(
			rng.start..mid,
			mid..rng.end
		)
	}

	pub fn is_floor(&self, tree_index: usize) -> bool {
		tree_index >= Self::floor_start(self.data.len())
	}

	pub fn node_left_child(&self, node: &NodePositionDescriptor) -> Option<NodePositionDescriptor> {
		if self.is_floor(node.tree_index) { None }
		else {
			Some(NodePositionDescriptor {
				tree_index: Self::left_child(node.tree_index),
				curated_segment: Self::half_split_range(&node.curated_segment).0
			})
		}
	}

	pub fn node_right_child(&self, node: &NodePositionDescriptor) -> Option<NodePositionDescriptor> {
		if self.is_floor(node.tree_index) { None }
		else {
			Some(NodePositionDescriptor {
				tree_index: Self::right_child(node.tree_index),
				curated_segment: Self::half_split_range(&node.curated_segment).1
			})
		}
	}

	pub(crate) fn left_child(i: usize) -> usize {
		2 * i + 1
	}

	pub(crate) fn right_child(i: usize) -> usize {
		2 * i + 2
	}

	pub(crate) fn parent(i: usize) -> usize {
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
		assert!(tree_size >= 4);

		tree_size / 2 - 1
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

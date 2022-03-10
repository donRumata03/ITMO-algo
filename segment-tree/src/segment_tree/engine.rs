use std::cmp::max;
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

enum RangeRelation {
	Inside,         // No elements outside
	Outside,        // No elements inside
	Intersection    // Some elements both inside and outside
}

impl<RE: ReductionElement, RO: ReductionOp<RE>> SegmentTreeEngine<RE, RO> {
	pub fn half_split_range(rng: &Range<usize>) -> (Range<usize>, Range<usize>) {
		let mid = (rng.start + rng.end) / 2;
		(
			rng.start..mid,
			mid..rng.end
		)
	}

	pub fn intersect_ranges(r1: &Range<usize>, r2: &Range<usize>) -> Range<usize> {
		max(r1.start, r2.start)..min(r1.end, r2.end)
	}


	// pub fn ranges_equal(r1: Range<usize>, r2: Range<usize>) {
	//
	// }

	pub fn contains_range(container: &Range<usize>, contained: &Range<usize>) -> bool {
		container.start <= contained.start && container.end >= contained.end
	}



	// fn range_relation(base: Range<usize>, analyzed: Range<usize>) -> RangeRelation {
	// 	if base.() {
	//
	// 	}
	//
	// 	let b = base == analyzed;
	//
	// 	todo!()
	// }

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

	pub fn left_child(i: usize) -> usize {
		2 * i + 1
	}

	pub fn right_child(i: usize) -> usize {
		2 * i + 2
	}

	pub fn parent(i: usize) -> Option<usize> {
		if i == 0 {None} else { Some((i - 1) / 2) }
	}

	pub fn smallest_pow_of_two_size(n: usize) -> usize { // TODO: 1. Try using 2 ^n - 1; 2. Try non-full tree with 2n elements
		msb(n)
			.map_or(0, |b| {
				2_usize.pow(if 2usize.pow(b as u32) == n {
					b
				} else { b + 1 }
					as u32 + 1)})
	}

	pub fn floor_start(tree_size: usize) -> usize {
		assert!(tree_size.is_power_of_two());
		assert!(tree_size >= 2);

		tree_size / 2 - 1
	}

	pub fn initial_element_tree_index(tree_size: usize, initial_index: usize) -> usize {
		Self::floor_start(tree_size) + initial_index
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

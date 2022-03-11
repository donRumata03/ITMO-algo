use std::cmp::max;
use std::ops::{Index, IndexMut};
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
pub trait SegmentTreeNode {
	fn neutral() -> Self;
}

pub struct SegmentTreeEngine<RE: ReductionElement, RO: ReductionOp<RE>, Node> {
	data: Vec<Node>,
	_re: PhantomData<RE>,
	_ro: PhantomData<RO>
}

enum RangeRelation {
	Inside,         // No elements outside
	Outside,        // No elements inside
	Intersection    // Some elements both inside and outside
}

impl<RE: ReductionElement, RO: ReductionOp<RE>, Node: SegmentTreeNode> SegmentTreeEngine<RE, RO, Node> {
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

	pub fn contains_range(container: &Range<usize>, contained: &Range<usize>) -> bool {
		container.start <= contained.start && container.end >= contained.end
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

	pub fn array_size(&self) -> usize {
		self.data.size() / 2
	}

	pub fn floor_start(&self) -> usize {
		self.data.size() / 2 - 1
	}

	pub fn initial_element_tree_index(&self, initial_index: usize) -> usize {
		self.floor_start() + initial_index
	}
}

impl<RE: ReductionElement, RO: ReductionOp<RE>, Node: SegmentTreeNode> SegmentTreeEngine<RE, RO, Node> {
	pub fn is_floor(&self, tree_index: usize) -> bool {
		tree_index >= self.floor_start()
	}

	fn floor_range(&self) -> Range<usize> {
		let start = self.floor_start();
		start..start + self.array_size()
	}

	pub fn node_left_child(&self, node: & NodePositionDescriptor) -> Option<NodePositionDescriptor> {
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

	// pub fn access_node(&mut self, node_descriptor: &NodePositionDescriptor) -> Node {
	// 	self.data
	// }


	pub fn fill_neutral(n: usize) -> Self {
		Self {
			data: std::iter::repeat(Node::neutral())
			.take( Self::smallest_pow_of_two_size(n))
			.collect(),
			_re: Default::default(),
			_ro: Default::default()
		}
	}

	pub fn decompose_into_segments() -> Vec<Node> {
		todo!()
	}

	pub fn visit() {
		todo!()
	}
}

impl<RE: ReductionElement, RO: ReductionOp<RE>, Node: SegmentTreeNode>
Index<NodePositionDescriptor> for SegmentTreeEngine<RE, RO, Node>
{
	type Output = Node;

	fn index(&self, index: NodePositionDescriptor) -> &Self::Output {
		&self.data[index.tree_index]
	}
}

impl<RE: ReductionElement, RO: ReductionOp<RE>, Node: SegmentTreeNode>
IndexMut<NodePositionDescriptor> for SegmentTreeEngine<RE, RO, Node>
{
	fn index_mut(&mut self, index: NodePositionDescriptor) -> &mut Self::Output {
		&mut self.data[index.tree_index]
	}
}

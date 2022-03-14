use std::cmp::max;
use std::iter::TrustedRandomAccessNoCoerce;
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

pub enum ChildnessType {
	LeftChild,
	RightChild
}

// impl NodePositionDescriptor {
//
// }

/// May contain only reduction information or may be also pending modification queries, for example.
/// Its content depends on implementation details of particular segment tree
pub trait SegmentTreeNode {
	fn neutral() -> Self;
}


/// There are a lot of variations of segment trees. But they all definitely have something in common.
/// SegmentTreeEngine is responsible for navigation on ST representation as contiguous memory.
///
/// When we traverse a segment tree, there are a couple of typical options:
/// — Searching for a particular element (given its index)
///
/// — Discovering into a segment (by going through `O(log(n))` nodes and dividing the segment into `O(log(n))` nodes)
///
/// — Conditional tree traversal (for example, doing binary search): performing decision which child to visit for each node
///
/// — Condition traversal with filtering: if node doesn't satisfy a certain predicate, don't step into it
///
/// For each option the `custom traverser` can do something with each element on its path
/// For example,
///
/// — «push» modification descriptors by splitting it into two identical parts and forwarding them to both children
///
/// — apply operation to the whole segment and reset parent's descriptors to `id`
///
/// The whole variety of traversals can be represented as top-to-down descent with predicates (with some standard presets).
/// But recursion can be optimized out for some special cases
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

	pub fn left_child_index(i: usize) -> usize {
		2 * i + 1
	}

	pub fn right_child_index(i: usize) -> usize {
		2 * i + 2
	}

	pub fn parent_index(i: usize) -> Option<usize> {
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


/// Impls for typical traversals
impl<RE: ReductionElement, RO: ReductionOp<RE>, Node: SegmentTreeNode> SegmentTreeEngine<RE, RO, Node> {

	///
	pub fn traverse_up_from_node_inclusive<F>(node: NodePositionDescriptor, f: F)
		where F: FnMut(NodePositionDescriptor)
	{
		if self.parent
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
				tree_index: Self::left_child_index(node.tree_index),
				curated_segment: Self::half_split_range(&node.curated_segment).0
			})
		}
	}

	pub fn node_right_child(&self, node: &NodePositionDescriptor) -> Option<NodePositionDescriptor> {
		if self.is_floor(node.tree_index) { None }
		else {
			Some(NodePositionDescriptor {
				tree_index: Self::right_child_index(node.tree_index),
				curated_segment: Self::half_split_range(&node.curated_segment).1
			})
		}
	}

	/// All layers that have parents (i. e. all excluding root node) start from odd indexes
	/// Node `0` is considered «right child» of nothing
	pub fn childness_type(&self, node: &NodePositionDescriptor) -> ChildnessType {
		assert_ne!(node.tree_index, 0);

		match node.tree_index % 2 {
			0 => ChildnessType::RightChild,
			1 => ChildnessType::LeftChild,
			_ => panic!()
		}
	}


	pub fn node_sibling(&self, node: &NodePositionDescriptor) -> Option<NodePositionDescriptor> {
		// Siblings always have exactly the same (and power-of-two) ranges for now
		let parent = self.node_parent(node)?;
		Some(
			match self.childness_type(node) {
				ChildnessType::LeftChild => self.node_right_child(&parent),
				ChildnessType::RightChild => self.node_left_child(&parent)
			}.unwrap()
		)
	}

	pub fn node_parent(&self, node: &NodePositionDescriptor) -> Option<NodePositionDescriptor> {
		// To get parent's range, we need to combine self with sibling (left or right)
		let parent_index = Self::parent_index(node.tree_index)?;

		let node_range_size = node.curated_segment.size();
		let parent_range = match self.childness_type(node) {
			ChildnessType::LeftChild => node.curated_segment.start..node.curated_segment.end + node_range_size,
			ChildnessType::RightChild => node.curated_segment.start - node_range_size..node.curated_segment.end
		};

		Some(NodePositionDescriptor{ tree_index: parent_index, curated_segment: parent_range })
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

	pub fn set_floor(&mut self, array: &Vec<Node>) {
		self.data[self.floor_range()].clone_from_slice(array);
	}

	pub fn rebuild_from_floor<F>(&mut self, combiner: F)
		where F: FnMut(Node, Node) -> Node
	{
		&mut self.data[..self.floor_start()]
			.iter_mut()
			.enumerate()
			.rev()
			.for_each(|(index, node)| {
				*node = combiner(
					self.data[Self::left_child_index(index)].clone(),
					self.data[Self::right_child_index(index)].clone()
				);
			})
		;
	}

	/// Segments in answer are sorted because recursion always starts from left
	pub fn decompose_into_segments_impl(
		&self,
		range: Range<usize>,
		root: NodePositionDescriptor,
		mut accumulator: Vec<NodePositionDescriptor>
	) -> Vec<NodePositionDescriptor> {

		todo!();

		accumulator
	}

	pub fn decompose_into_segments(&self, range: Range<usize>) -> Vec<NodePositionDescriptor> {
		let mut accumulator = Vec::new();

		self.decompose_into_segments_impl(range, );

		accumulator
	}

	pub fn reduce() -> Option<Node> {
		todo!()
	}

	pub fn traverse(/* visitor: Fn(&mut Node) -> bool */) {
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

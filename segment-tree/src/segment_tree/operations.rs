use super::*;

pub trait ReductionElement: Clone {

}

pub trait ReductionOp<T: ReductionElement>: Clone {
    fn neutral() -> T;
    fn apply(left: T, right: T) -> T;
}

pub trait CommutativeOp<T: ReductionElement>: ReductionOp<T> {}
pub trait DistributiveRelativeTo<T: ReductionElement>: ReductionOp<T> {}



///_____________________________________________________________________________

/// Type that represents set of possible modifications of elements
pub trait ModificationDescriptor<RE: ReductionElement>: Clone {
    fn identity() -> Self;
    fn apply(&self, argument: RE) -> RE;
}

/// Different kinds of queries { modification, reduction } × { segment, element }
#[derive(Debug, Clone)]
pub struct ElementModificationQuery<RE: ReductionElement, MD: ModificationDescriptor<RE>> {
    pub position: usize,
    pub mqd: MD,
    _re: PhantomData<RE>
}

impl<RE: ReductionElement, MD: ModificationDescriptor<RE>> ElementModificationQuery<RE, MD> {
    pub fn new(position: usize, mqd: MD) -> Self {
        Self {
            position,
            mqd,
            _re: Default::default()
        }
    }
}

// impl<RE> ModificationQuery for ElementModificationQuery<RE> {}

#[derive(Debug, Clone)]
pub struct SegmentModificationQuery<RE: ReductionElement, MD: ModificationDescriptor<RE>> {
    pub segment: Range<usize>,
    pub mqd: MD,
    _re: PhantomData<RE>
}

impl<RE: ReductionElement, MD: ModificationDescriptor<RE>> From<ElementModificationQuery<RE, MD>> for SegmentModificationQuery<RE, MD> {
    fn from(q: ElementModificationQuery<RE, MD>) -> Self {
        SegmentModificationQuery::new(q.position..q.position + 1, q.mqd)
    }
}

impl<RE: ReductionElement, MD: ModificationDescriptor<RE>> SegmentModificationQuery<RE, MD> {
    pub fn new(segment: Range<usize>, mqd: MD) -> Self {
        Self {
            segment,
            mqd,
            _re: Default::default()
        }
    }
}

#[derive(Debug, Clone)]
pub struct ElementReductionQuery<RE: ReductionElement, RO: ReductionOp<RE>> {
    pub element_index: usize,
    _re: PhantomData<RE>,
    _ro: PhantomData<RO>
}

impl<RE: ReductionElement, RO: ReductionOp<RE>> ElementReductionQuery<RE, RO> {
    pub fn new(element_index: usize) -> Self {
        Self {
            element_index,
            _re: Default::default(),
            _ro: Default::default()
        }
    }
}

impl<RE: ReductionElement, RO: ReductionOp<RE>> From<ElementReductionQuery<RE, RO>> for SegmentReductionQuery<RE, RO> {
    fn from(q: ElementReductionQuery<RE, RO>) -> Self {
        Self::new(q.element_index..q.element_index + 1)
    }
}


#[derive(Debug, Clone)]
pub struct SegmentReductionQuery<RE: ReductionElement, RO: ReductionOp<RE>> {
    pub segment: Range<usize>,
    _re: PhantomData<RE>,
    _ro: PhantomData<RO>
}


impl<RE: ReductionElement, RO: ReductionOp<RE>> SegmentReductionQuery<RE, RO> {
    pub fn new(segment: Range<usize>) -> Self {
        Self {
            segment,
            _re: Default::default(),
            _ro: Default::default()
        }
    }
}

/// Traits for a data structure answering different type of queries
pub trait ElementModifier<RE: ReductionElement, MD: ModificationDescriptor<RE>> {
    fn modify_element(&mut self, q: &ElementModificationQuery<RE, MD>);
}

pub trait SegmentModifier<RE: ReductionElement, MD: ModificationDescriptor<RE>> {
    fn modify_segment(&mut self, q: &SegmentModificationQuery<RE, MD>);
}

pub trait ElementReducer<RE: ReductionElement, RO: ReductionOp<RE>> {
    fn reduce_element(&mut self, q: &ElementReductionQuery<RE, RO>) -> RE;
}

pub trait SegmentReducer<RE: ReductionElement, RO: ReductionOp<RE>> {
    fn reduce_segment(&mut self, q: &SegmentReductionQuery<RE, RO>) -> RE;
}


/// Composition and recounting (for both reduction and modification on segment)

pub trait ComposableModificationDescriptor<RE: ReductionElement>: ModificationDescriptor<RE> {
    fn compose(old: &Self, new: &Self) -> Self;
}


pub trait RecountableAfterMassApplication<RE: ReductionElement, MD: ModificationDescriptor<RE>> {
    fn recount(reduction_element: &mut RE, query: MD, len: usize);
}

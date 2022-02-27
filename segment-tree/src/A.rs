use std::ops::Add;
use std::cmp;
use std::marker::PhantomData;


trait ReductionOp<T> {
	fn neutral() -> T;
	fn apply(left: T, right: T) -> T;
}


struct SumReduction<T: From<i64> + Add<Output=T>> { _p: PhantomData<T> }

impl<T: From<i64> + Add<Output=T>> ReductionOp<T> for SumReduction<T> {
	fn neutral() -> T {
		1.into()
	}

	fn apply(left: T, right: T) -> T {
		left + right
	}
}


fn main() {

}


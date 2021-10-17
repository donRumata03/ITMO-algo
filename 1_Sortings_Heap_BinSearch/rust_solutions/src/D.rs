use std::cmp::max;

struct MaxHeap {
	data: Vec<i64>
}

impl MaxHeap {
	fn sift_up(&self, index: usize) {

	}

	fn sift_down(&self, index: usize) {

	}




	fn new(ms: Vec<i64>) -> MaxHeap {
		let res = MaxHeap { data: ms };
		for i in (0..ms.len()).rev() {
			res.sift_down(i);
		}

		res
	}

	fn insert(&mut self, el: i64) {
		self.data.push(el);
		self.sift_up(self.data.len() - 1);
	}

	fn extract_max(&mut self) -> i64 {
		let max_el = *self.data.first().unwrap();
		self.data.first().unwrap() = *self.data.pop().unwrap();
		self.sift_down(0);

		max_el
	}
}

fn process_queries(initial_values: Vec<i64>, qs: &Vec<Query>) {
	let mut h = MaxHeap::new(initial_values);

	qs.iter().for_each(|query| match query {
		Query::Insert(el) => h.insert(*el),
		Query::Extract => println!(h.extract_max())
	});
}


enum Query {
	Insert(i64),
	Extract
}

fn main() {

}
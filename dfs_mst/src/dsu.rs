pub struct DSU {
	parent: Vec<usize>,
	rank: Vec<usize>,
}

impl DSU {
	pub fn new(n: usize) -> Self {
		DSU {
			parent: (0..n).collect(),
			rank: vec![0; n],
		}
	}

	pub fn find(&mut self, v: usize) -> usize {
		if self.parent[v] == v {
			v
		} else {
			self.parent[v] = self.find(self.parent[v]);
			self.parent[v]
		}
	}

	pub fn union(&mut self, a: usize, b: usize) {
		let a = self.find(a);
		let b = self.find(b);
		if a != b {
			if self.rank[a] < self.rank[b] {
				self.parent[a] = b;
			} else {
				self.parent[b] = a;
				if self.rank[a] == self.rank[b] {
					self.rank[a] += 1;
				}
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_dsu() {
		let mut dsu = DSU::new(5);
		assert_eq!(dsu.find(0), 0);
		assert_eq!(dsu.find(1), 1);
		assert_eq!(dsu.find(2), 2);
		assert_eq!(dsu.find(3), 3);
		assert_eq!(dsu.find(4), 4);
		dsu.union(0, 1);
		assert_eq!(dsu.find(0), 0);
		assert_eq!(dsu.find(1), 0);
		assert_eq!(dsu.find(2), 2);
		assert_eq!(dsu.find(3), 3);
		assert_eq!(dsu.find(4), 4);
		dsu.union(2, 3);
		assert_eq!(dsu.find(0), 0);
		assert_eq!(dsu.find(1), 0);
		assert_eq!(dsu.find(2), 2);
		assert_eq!(dsu.find(3), 2);
		assert_eq!(dsu.find(4), 4);
		dsu.union(0, 2);
		assert_eq!(dsu.find(0), 0);
		assert_eq!(dsu.find(1), 0);
		assert_eq!(dsu.find(2), 0);
		assert_eq!(dsu.find(3), 0);
		assert_eq!(dsu.find(4), 4);
		dsu.union(4, 0);
		assert_eq!(dsu.find(0), 0);
		assert_eq!(dsu.find(1), 0);
		assert_eq!(dsu.find(2), 0);
		assert_eq!(dsu.find(3), 0);
		assert_eq!(dsu.find(4), 0);
	}
}

const HASH_BASE: u64 = 163;


pub struct StringHasher {
	prefix_hashes: Vec<u64>, // prefix_hashes[i] = hash(s[0..i)), i.e. hash of the prefix of length i. E.g. prefix_hashes[0] = hash("")
	powers: Vec<u64>, // powers[i] = HASH_BASE^i
}

impl StringHasher {
	pub fn new(s: &str) -> Self {
		let mut prefix_hashes = vec![0; s.len() + 1];
		let mut powers = vec![1; s.len() + 1];
		for i in 1..=s.len() {
			prefix_hashes[i] = prefix_hashes[i - 1] * HASH_BASE + s.as_bytes()[i - 1] as u64;
			powers[i] = powers[i - 1] * HASH_BASE;
		}
		Self { prefix_hashes, powers }
	}

	fn n(&self) -> usize { self.prefix_hashes.len() - 1 }

	/// [l, r)
	/// We could divide by powers[l] to get the hash of the substring, but that's too tedious.
	/// So we just multiply by powers[s.len() - l] to get substring hashes properly compared
	pub fn substring_hash(&self, l: usize, r: usize) -> u64 {
		(self.prefix_hashes[r] - self.prefix_hashes[l] * self.powers[r - l]) // this gives hash(s[l..r))
			// .overflowing_mul(self.powers[self.n() - (r - l + 1)]).0
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_string_hasher() {
		let s = "abacaba";
		let hasher = StringHasher::new(s);
		assert_eq!(hasher.substring_hash(0, 1), 'a' as u64);
		assert_eq!(hasher.substring_hash(1, 2), 'b' as u64);
		assert_eq!(hasher.substring_hash(2, 3), 'a' as u64);
		assert_eq!(hasher.substring_hash(3, 4), 'c' as u64);
		assert_eq!(hasher.substring_hash(4, 5), 'a' as u64);
		assert_eq!(hasher.substring_hash(5, 6), 'b' as u64);
		assert_eq!(hasher.substring_hash(6, 7), 'a' as u64);
	}

	// Hashes of equal substrings should be equal
	// And hashes of different substrings should be different
	#[test]
	fn test_string_hasher_equal_substrings() {
		let s = "abacaba";
		let hasher = StringHasher::new(s);

		assert_eq!(hasher.substring_hash(0, 1), hasher.substring_hash(6, 7));
		assert_eq!(hasher.substring_hash(0, 3), hasher.substring_hash(4, 7));

		assert_ne!(hasher.substring_hash(0, 1), hasher.substring_hash(1, 2));
		assert_ne!(hasher.substring_hash(0, 3), hasher.substring_hash(1, 4));
	}
}

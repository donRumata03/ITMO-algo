use std::cmp::min;

/// Encoding policy: per byte, not per character.
pub fn prefix_function(s: &str) -> Vec<usize> {
	let mut pi = vec![0; s.len()];
	let mut k = 0;
	for i in 1..s.len() {
		while k > 0 && s.as_bytes()[k] != s.as_bytes()[i] {
			k = pi[k - 1];
		}
		if s.as_bytes()[k] == s.as_bytes()[i] {
			k += 1;
		}
		pi[i] = k;
	}
	pi
}

pub fn z_function(s: &str) -> Vec<usize> {
	let mut z = vec![0; s.len()];
	let mut l = 0;
	let mut r = 0;
	for i in 1..s.len() {
		if i <= r {
			z[i] = min(r - i + 1, z[i - l]);
		}
		while i + z[i] < s.len() && s.as_bytes()[z[i]] == s.as_bytes()[i + z[i]] {
			z[i] += 1;
		}
		if i + z[i] - 1 > r {
			l = i;
			r = i + z[i] - 1;
		}
	}
	z
}

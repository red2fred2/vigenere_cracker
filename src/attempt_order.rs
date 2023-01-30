/*
 * I have no idea why it's so hard to traverse a matrix like this. At least it
 * runs in like 200 us.
 */

pub struct AttemptOrder {
	combination: Vec<usize>,
	combination_num: usize,
	key_length: usize,
	num_letters: usize,
	num_changes: usize,
	is_first: bool,
}

impl AttemptOrder {
	/**
	 * Returns a new attempt order generator
	 */
	pub fn new(key_length: usize, num_letters: usize) -> Self {
		let combination = vec![0; key_length];
		let combination_num = 0;
		let num_changes = 0;
		let is_first = true;
		AttemptOrder {combination, combination_num, key_length, num_letters, num_changes, is_first}
	}

	/**
	 * Adds a partial combination to the full combination
	 */
	fn add_combination_part(&mut self, combo: &Vec<usize>) {
		for (i, part) in combo.iter().enumerate() {
			self.combination[i] += part;
		}
	}

	/**
	 * Gets a partial combination
	 */
	fn get_combination_part(&self, change_num: usize, n: usize) -> Vec<usize> {
		let rows = self.key_length - change_num + 1;
		let cols = self.num_letters - 1;

		let mut part = vec![0; rows];
		part[n % rows] = (n / rows) % cols + 1;

		part
	}

	/**
	 * Uses global iteration to find this part's n
	 */
	fn get_part_n(&self, change_num: usize) -> usize {
		let mut n = self.combination_num;

		for i in change_num..self.num_changes {
			n /= (self.key_length - i) * (self.num_letters - 1);
		}

		n
	}

	/**
	 * Checks if this attempt is the last with this num_changes
	 */
	fn is_last(&self) -> bool {
		if self.num_changes == 0 {
			return true;
		}

		let mut has_seen_changes: bool = false;

		for p in &self.combination {
			if *p > 0 {
				has_seen_changes = true;

				if *p != self.num_letters - 1 {
					return false;
				}
			} else if has_seen_changes {
				return false;
			}
		}

		true
	}

	/**
	 * Stretch a vector to fit over the key length
	 */
	fn stretch_vec(&self, vec: &mut Vec<usize>) {
		for (i, e) in self.combination.iter().enumerate() {
			if *e > 0 {
				vec.insert(i, 0);
			}
		}
	}
}

impl Iterator for AttemptOrder {
	type Item = Vec<usize>;

	/**
	 * Gets the next iteration of this combination
	 */
	fn next(&mut self) -> Option<Self::Item> {
		// Special case for the first attempt
		if self.is_first {
			self.is_first = false;
			self.combination_num = 0;
			return Some(self.combination.clone());
		}

		// Check if this is the last of this num_changes
		if self.is_last() {
			// Check if it's the very last one
			if self.num_changes == self.key_length {
				return None;
			}

			self.combination_num = 0;
			self.num_changes += 1;
		} else {
			self.combination_num += 1;
		}

		// Generate a new combination
		self.combination = vec![0; self.key_length];

		for i in (0..self.num_changes).rev() {
			let change_num = self.num_changes - i;
			let n = self.get_part_n(change_num);

			let mut part = self.get_combination_part(change_num, n);
			if part.len() < self.key_length {
				self.stretch_vec(&mut part);
			}

			self.add_combination_part(&part);
		}

		Some(self.combination.clone())

	}
}

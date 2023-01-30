/**
 * A representation of a dictionary's possible words as a tree of letters
 */
pub struct DictTree {
	head: Node,
}

struct Node {
	children: Vec<Option<Box<Node>>>
}

type Encoded = Vec<u8>;
type Dict = Vec<Vec<u8>>;

impl DictTree {
	/**
	 * Creates a new DictTree from a Dict
	 */
	pub fn new(dict: Dict) -> Self {
		let children = std::iter::repeat_with(|| None).take(26).collect();
		let mut head = Node{children};

		// Add all the words
		for mut word in dict {
			head.add(&mut word);
		}

		DictTree{head}
	}

	/**
	 * Find out if a string exists in this dictionary tree
	 */
	pub fn exists(&self, string: &Encoded) -> bool {
		let mut str = string.clone();
		self.head.exists(&mut str)
	}
}

impl Node {
	/**
	 * Find out if a string exists in this dictionary tree
	 */
	fn exists(&self, string: &mut Encoded) -> bool {
		if string.len() == 1 {
			let index = usize::from(string[0]);
			self.children[index].is_some()
		} else {
			let front = string.remove(0);
			let index = usize::from(front);

			match &self.children[index] {
				Some(node) => node.exists(string),
				None => false
			}
		}
	}

	/**
	 * Adds a string to the tree
	 */
	fn add(&mut self, string: &mut Encoded) {
		if string.len() > 0 {
			let index = usize::from(string.remove(0));

			match &mut self.children[index] {
				Some(node) => node.add(string),
				None => {
					let children = std::iter::repeat_with(|| None).take(26).collect();
					self.children[index] = Some(Box::new(Node{children}));

					self.children[index].as_mut().unwrap().add(string);
				}
			}
		}
	}
}

type Encoded = Vec<u8>;
type Dict = Vec<Encoded>;

/**
 * Flips around the Option by returning None if any element of the input is None
 */
fn all_or_nothing(input: &Vec<Option<u8>>) -> Option<Encoded> {
	if input.iter().any(|e| e.is_none()) {
		None
	} else {
		Some(input.iter().map(|e| e.unwrap()).collect())
	}
}

/**
 * Checks if an attempted decryption is a possible solution
 */
fn check_attempt(attempt: &Encoded, first_word_dict: &Dict) -> bool {
	let first_word_length = first_word_dict[0].len();

	for word in first_word_dict {
		let mut word_possible = true;

		for i in 0..first_word_length {
			let letter = word[i];

			// check if it fails at this letter
			if letter != attempt[i] {
				word_possible = false;
				break;
			}
		}

		// If any of the words were possible, just return true
		if word_possible {
			return true;
		}
	}

	return false;
}

/**
 * Decodes a number 0 to 25 as a character a-z. Returns None when out of range.
 */
fn decode_char(code: &u8) -> Option<char> {
	char::from_digit(u32::from(code + 10), 36)
}

/**
 * Decodes a Vec of numbers 0 to 25 and return a lowercase string. Returns None
 * when any numbers are out of range.
 */
fn decode_str(code: &Encoded) -> Option<String> {
	code.iter().map(|c| decode_char(c)).collect()
}

/**
 * Tries to decode a string and crashes if it can't
 */
fn decode(code: &Encoded) -> String {
	decode_str(code).expect("Failed to decode string")
}

/**
 * Decrypts one u8 character
 */
fn decrypt_char(input: &u8, key: &u8) -> u8 {
	(input + 26 - key) % 26
}

/**
 * Decrypts a Vec of u8 characters
 */
fn decrypt_str(input: &Encoded, key: &Encoded) -> Encoded {
	input.iter().enumerate().map(
		|(i, c)| decrypt_char(c, &key[i % key.len()])
	).collect()
}

/**
 * Encodes a unicode as a number 0 to 25 when the character is a-z|A-Z. Returns
 * None when the character is not alphabetical.
 */
fn encode_char(message: &char) -> Option<u8> {
	match message.to_digit(36) {
		Some(n) => if n < 10 {
			None
		} else {
			Some(u8::try_from(n).unwrap() - 10)
		},
		None => None
	}
}

/**
 * Encodes a String as a Vec of u8s valued 0 to 25. Characters will be None when
 * not alphabetical.
 */
fn encode_str(message: &String) -> Vec<Option<u8>> {
	message.chars().map(|c| encode_char(&c)).collect()
}

/**
 * Tries to encode a string and crashes if it doesn't work
 */
fn encode(message: &String) -> Encoded {
	let stripped = strip_message(message);
	let encoded = encode_str(&stripped);
	all_or_nothing(&encoded).expect("Failed to encode string")
}

/**
 * Encrypts one u8 character
 */
fn encrypt_char(input: &u8, key: &u8) -> u8 {
	(input + key) % 26
}

/**
 * Encrypts a Vec of u8 characters
 */
#[allow(dead_code)]
fn encrypt_str(input: &Encoded, key: &Encoded) -> Encoded {
	input.iter().enumerate().map(
		|(i, c)| encrypt_char(c, &key[i % key.len()])
	).collect()
}

/**
 * Filters a given dictionary to only include words of a certain length
 */
fn filter_dictionary(dict: &Vec<String>, length: usize) -> Vec<String> {
	dict.iter().filter(|w| w.len() == length).map(|w| w.clone()).collect()
}

/**
 * Finds the best frequecy match offsets from best to worst
 */
fn find_best_offsets(a: &Vec<f32>, b: &Vec<f32>) -> Vec<u8> {
	// Lowest fitness is best
	let mut fitness_list: Vec<(f32, usize)> = Vec::new();

	for offset in 0..26 {
		let mut fitness = (0.0, offset);

		for i in 0..26 {
			let index = (i + offset) % 26;
			let (fit, _) = fitness;
			let diff = (a[i] - b[index]).abs();

			fitness = (fit + diff, offset);
		}
		fitness_list.push(fitness);
	}

	// println!("Fitness list: {:?}", fitness_list);

	fitness_list.sort_unstable_by(
		|(a, _), (b, _)| a.partial_cmp(b).unwrap()
	);

	fitness_list.iter().map(|(_, i)| u8::try_from(*i).unwrap()).collect()
}

/**
 * Generate the frequencies of letters in the dictionary
 */
fn gen_dict_freqs(dict: &Dict) -> Vec<f32> {
	let mut table: Vec<f32> = vec![0.0; 26];

	for word in dict {
		for c in word {
			let index = usize::from(*c);
			table[index] += 1.0;
		}
	}
	let total: f32 = table.iter().sum();

	table.iter().map(|e| e / total).collect()
}

/**
 * Generate the frequencies of letters in the encoded string
 */
fn gen_freqs(string: &Encoded) -> Vec<f32> {
	let mut table: Vec<f32> = vec![0.0; 26];

	for c in string {
		let index = usize::from(*c);
		table[index] += 1.0;
	}
	let total: f32 = table.iter().sum();

	table.iter().map(|e| e / total).collect()
}

/**
 * Reads in a dictionary from a file path
 */
fn get_dictionary(file_path: &str) -> Vec<String> {
	std::fs::read_to_string(file_path)
	.expect("Failed to read dictionary file")
	.split_whitespace()
	.map(|w| w.to_string())
	.collect()
}

/**
 * Strides over an encoded string and returns one that matches the repeating
 * pattern of a key
 */
fn stride(input: &Encoded, stride: usize, offset: usize) -> Encoded {
	let mut output = Vec::new();

	let length = input.len();
	for i in (offset..length).step_by(stride) {
		output.push(input[i]);
	}

	output
}

/**
 * Makes all characters lowercase then strips out non a-z ones.
 */
fn strip_message(message: &String) -> String {
	message.to_lowercase().chars().filter(|c| c >= &'a' && c <= &'z').collect()
}

fn main() {
	// Set inputs
	let raw_ciphertext = "VVVLZWWPBWHZDKBTXLDCGOTGTGRWAQWZSDHEMXLBELUMO".to_string();
	let pw_len = 7;
	let first_word_len = 6;

	// Dictionaries
	let raw_dict = get_dictionary("./dictionary.txt");
	let filtered_dict = filter_dictionary(&raw_dict, first_word_len);

	// Encode
	let ciphertext = encode(&raw_ciphertext);

	let full_dict: Dict = raw_dict.iter().map(|w| encode(w)).collect();
	let first_word_dict: Dict = filtered_dict.iter().map(|w| encode(w)).collect();

	// Find the best keys to try
	let dict_freqs = gen_dict_freqs(&full_dict);
	let mut best_keys: Vec<Vec<u8>> = Vec::new();

	for key_part in 0..pw_len {
		let relevant_ciphertext = stride(&ciphertext, pw_len, key_part);
		let ciphertext_freqs = gen_freqs(&relevant_ciphertext);

		best_keys.push(find_best_offsets(&dict_freqs, &ciphertext_freqs));
		// .iter().map(|e| decode_char(e).unwrap()).collect());
	}

	// Attempt decryption
	let key: Encoded = vec![best_keys[0][0], best_keys[1][0]];
	let attempt = decrypt_str(&ciphertext, &key);

	let isGood = check_attempt(&attempt, &first_word_dict);

	println!("{:?}", decode(&attempt));
}

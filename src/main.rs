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
 * Finds the decode order most likely to run into impossible characters
 */
fn find_best_decode_order(weed_outs: &Vec<Vec<u8>>) -> Vec<usize> {
	let mut num_weeds: Vec<usize> = weed_outs.iter().map(|e| e.len() + 1).collect();
	let mut order = Vec::new();

	for _ in 0..num_weeds.len() {
		let best = *num_weeds.iter().max().unwrap();
		let best_pos = num_weeds.iter().position(|e| e == &best).unwrap();
		num_weeds[best_pos] = 0;
		order.push(best_pos);
	}

	order
}

/**
 * For the first word, finds which letters do not occur at certain positions in
 * the dictionary. Each letter weeded out will save 26^len-1 runs. All dictionary
 * items must be the same length.
 */
fn find_weed_out_letters(dictionary: &Dict, first_word_length: usize) -> Vec<Vec<u8>> {
	let mut weed_out = Vec::new();

	for pos in 0..first_word_length {
		let mut table = vec![true; 26];

		for word in dictionary {
			let letter = word[pos];
			table[usize::from(letter)] = false;
		}

		let alphabet_len: u8 = 26;
		weed_out.push((0..alphabet_len).filter(|c| table[usize::from(*c)]).collect());
	}

	weed_out
}

/**
 * Generates the first password of a certain length
 */
fn gen_first_pw(length: usize) -> Vec<u8> {
	vec![0; length]
}

/**
 * Generates the next password attempt
 */
fn gen_next_pw(pw: &mut Vec<u8>) {
	let len = pw.len();

	for n in 1..=len {
		match pw[len-n] {
			25 => pw[len-n] = 0,
			_ => {
				pw[len-n] += 1;
				break;
			}
		};
	}
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
 * Makes all characters lowercase then strips out non a-z ones.
 */
fn strip_message(message: &String) -> String {
	message.to_lowercase().chars().filter(|c| c >= &'a' && c <= &'z').collect()
}

fn main() {
	// The keys are all shorter than their respective first word length. Finding
	// the word should be enough to confidently find the key.
	let first_word_length: usize = 13;
	let key_length: usize = 2;

	// let message = encode(&"Slugmaballs".to_string());
	// let key = encode(&"penis".to_string());

	// let encrypted = encrypt_str(&message, &key);
	// println!("{:?}", decode(&encrypted));

	// let decrypted = decrypt_str(&encrypted, &key);
	// println!("{:?}", decode(&decrypted));

	let raw_dict = get_dictionary("./dictionary.txt");
	let decoded_dict = filter_dictionary(&raw_dict, first_word_length);
	let dict: Dict = decoded_dict.iter().map(|w| encode(w)).collect();

	let weed_outs = find_weed_out_letters(&dict, first_word_length);
	let best_order = find_best_decode_order(&weed_outs);

	println!("{:?}", weed_outs);
	println!("{:?}", best_order);
}

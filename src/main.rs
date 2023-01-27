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
 * Generates the first password of a certain length
 */
fn gen_first_pw(length: usize) -> Vec<u8> {
	vec![0; length]
}

/**
 * Generate the frequencies of letters in the dictionary
 */
fn gen_freqs(dict: &Dict) -> Vec<f32> {
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
	let pw_len = 2;
	let first_word_len = 6;

	// let message = encode(&"Slugmaballs".to_string());
	// let key = encode(&"penis".to_string());

	// let encrypted = encrypt_str(&message, &key);
	// println!("{:?}", decode(&encrypted));

	// let decrypted = decrypt_str(&encrypted, &key);
	// println!("{:?}", decode(&decrypted));

	let raw_dict = get_dictionary("./dictionary.txt");
	let full_dict: Dict = raw_dict.iter().map(|w| encode(w)).collect();
	let filtered_dict = filter_dictionary(&raw_dict, 9);
	let first_word_dict: Dict = filtered_dict.iter().map(|w| encode(w)).collect();

	let freqs = gen_freqs(&full_dict);

	println!("{:?}", freqs);

	// println!("{:?}", encoded_dict[0]);

	// let mut pw = gen_first_pw(2);

	// let num_pws: u64 = 26*26*26*26*26*26*26-1;

	// for _ in 0..num_pws {
	// 	gen_next_pw(&mut pw);
	// }

	// println!("{:?}", pw);
}

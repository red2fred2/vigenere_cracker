#![feature(test)]
extern crate test;

pub mod attempt_order;

use std::fs::File;
use std::io::prelude::*;
use serde_json;

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
 * Gets a key from the best keys list and a combination
 */
fn choose_key(best_keys: &Vec<Vec<u8>>, combination: &Vec<usize>) -> Encoded {
	let mut key = Vec::new();

	for (i, c) in combination.iter().enumerate() {
		let key_part = best_keys[i][*c];
		key.push(key_part);
	}

	key
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
#[allow(dead_code)]
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

fn write_dict_freqs(freqs: &Vec<f32>) -> std::io::Result<()> {
	let mut file = File::create("frequencies.json")?;

	let json = serde_json::to_string(&freqs)?;
	let mut data: Vec<u8> = Vec::new();

	write!(&mut data, "{}", json)?;
	file.write(&data)?;

	Ok(())
}

fn read_dict_freqs() -> std::io::Result<Vec<f32>> {
	let mut file = File::open("frequencies.json")?;

	let mut data = Vec::<u8>::new();
	file.read_to_end(&mut data)?;

	let freqs = serde_json::from_slice(&data)?;

	Ok(freqs)
}

/**
 * Writes the first word dictionary cache files
 */
fn write_fwds(dict: &Dict, length: usize) -> std::io::Result<Dict> {
	let mut dicts_by_length = vec![Vec::new();3];

	for word in dict {
		let len = word.len();

		match dicts_by_length.get_mut(len) {
			Some(d) => d.push(word.clone()),
			None => {
				dicts_by_length.insert(len, Vec::new());
				dicts_by_length[len].push(word.clone());
			}
		};
	}

	for (i, d) in dicts_by_length.iter().enumerate() {
		let path = format!("dict{}.json", i);
		let mut file = File::create(path)?;

		let json = serde_json::to_string(&d)?;
		let mut data: Vec<u8> = Vec::new();
		write!(&mut data, "{}", json)?;

		file.write(&data)?;
	}

	Ok(dicts_by_length[length].clone())
}

fn read_fwd(length: usize) -> std::io::Result<Dict> {
	let path = format!("dict{}.json", length);
	let mut file = File::open(path)?;

	let mut data = Vec::<u8>::new();
	file.read_to_end(&mut data)?;

	let freqs = serde_json::from_slice(&data)?;

	Ok(freqs)
}

fn main() -> std::io::Result<()> {
	let start = std::time::Instant::now();

	// Set inputs
	let dictionary_file = "./dictionary.txt";
	let raw_ciphertext = "VVVLZWWPBWHZDKBTXLDCGOTGTGRWAQWZSDHEMXLBELUMO".to_string();
	let pw_len = 7;
	let first_word_len = 13;

	let ciphertext = encode(&raw_ciphertext);

	// Get first word dictionary
	let dict_read = read_fwd(first_word_len);
	let dict = match dict_read {
		Ok(d) => d,
		_ => {
			let raw_dict = get_dictionary(dictionary_file);
			let dict = &raw_dict.iter().map(|w| encode(w)).collect();
			write_fwds(dict, first_word_len)?
		}
	};

	// Get letter frequencies
	let freqs_read = read_dict_freqs();
	let dict_freqs = match freqs_read {
		Ok(freqs) => freqs,
		_ => {
			let raw_dict = get_dictionary(dictionary_file);
			let encoded = raw_dict.iter().map(|w| encode(w)).collect();
			let freqs = gen_dict_freqs(&encoded);
			write_dict_freqs(&freqs)?;

			freqs
		}
	};

	let mut best_keys: Vec<Vec<u8>> = Vec::new();

	for key_part in 0..pw_len {
		let relevant_ciphertext = stride(&ciphertext, pw_len, key_part);
		let ciphertext_freqs = gen_freqs(&relevant_ciphertext);

		best_keys.push(find_best_offsets(&dict_freqs, &ciphertext_freqs));
	}

	// Attempt decryption
	let order = attempt_order::AttemptOrder::new(pw_len, 26);

	for combination in order {
		let key = choose_key(&best_keys, &combination);
		let attempt = decrypt_str(&ciphertext, &key);

		if check_attempt(&attempt, &dict) {
			println!("{} -> {}", decode(&key), decode(&attempt).to_uppercase());
			break;
		}
	}

	println!("It took {}ms", start.elapsed().as_millis());

	Ok(())
}

// Benchmarks
#[cfg(test)]
mod tests {
	use super::*;
    use test::Bencher;

	#[bench]
	fn bench_get_dictionary(b: &mut Bencher) {
		b.iter(|| get_dictionary("./dictionary.txt"))
	}

	#[bench]
	fn bench_filter_dictionary(b: &mut Bencher) {
		let dict = get_dictionary("./dictionary.txt");

		b.iter(|| filter_dictionary(&dict, 6))
	}

	#[bench]
	fn bench_encode(b: &mut Bencher) {
		let message = "PSPDYLOAFSGFREQKKPOERNIYVSDZSUOVGXSRRIPWERDIPCFSDIQZIASEJVCGXAYBGYXFPSREKFMEXEBIYDGFKREOWGXEQSXSKXGYRRRVMEKFFIPIWJSKFDJMBGCC".to_string();

		b.iter(|| encode(&message))
	}

	#[bench]
	fn bench_gen_dict_freqs(b: &mut Bencher) {
		let dictionary_file = "./dictionary.txt";
		let raw_dict = get_dictionary(dictionary_file);
		let dict: Dict = raw_dict.iter().map(|w| encode(w)).collect();

		b.iter(|| gen_dict_freqs(&dict))
	}

	#[bench]
	fn bench_stride(b: &mut Bencher) {
		let input = encode(&"PSPDYLOAFSGFREQKKPOERNIYVSDZSUOVGXSRRIPWERDIPCFSDIQZIASEJVCGXAYBGYXFPSREKFMEXEBIYDGFKREOWGXEQSXSKXGYRRRVMEKFFIPIWJSKFDJMBGCC".to_string());

		b.iter(|| stride(&input, 2, 1))
	}

	#[bench]
	fn bench_gen_freqs(b: &mut Bencher) {
		let string = encode(&"PSPDYLOAFSGFREQKKPOERNIYVSDZSUOVGXSRRIPWERDIPCFSDIQZIASEJVCGXAYBGYXFPSREKFMEXEBIYDGFKREOWGXEQSXSKXGYRRRVMEKFFIPIWJSKFDJMBGCC".to_string());

		b.iter(|| gen_freqs(&string))
	}

	#[bench]
	fn bench_find_best_offsets(b: &mut Bencher) {
		let text = encode(&"PSPDYLOAFSGFREQKKPOERNIYVSDZSUOVGXSRRIPWERDIPCFSDIQZIASEJVCGXAYBGYXFPSREKFMEXEBIYDGFKREOWGXEQSXSKXGYRRRVMEKFFIPIWJSKFDJMBGCC".to_string());
		let dict: Dict = get_dictionary("./dictionary.txt").iter().map(|w| encode(w)).collect();
		let dict_freqs = gen_dict_freqs(&dict);
		let text_freqs = gen_freqs(&text);

		b.iter(|| find_best_offsets(&dict_freqs, &text_freqs))
	}

	#[bench]
	fn bench_order(b: &mut Bencher) {
		let mut order = attempt_order::AttemptOrder::new(5, 26);

		b.iter(|| order.next())
	}

	#[bench]
	fn bench_decrypt_str(b: &mut Bencher) {
		let input = encode(&"PSPDYLOAFSGFREQKKPOERNIYVSDZSUOVGXSRRIPWERDIPCFSDIQZIASEJVCGXAYBGYXFPSREKFMEXEBIYDGFKREOWGXEQSXSKXGYRRRVMEKFFIPIWJSKFDJMBGCC".to_string());
		let key: Encoded = vec![5, 20];

		b.iter(|| decrypt_str(&input, &key))
	}

	#[bench]
	fn bench_check_attempt(b: &mut Bencher) {
		let attempt = encode(&"PSPDYLOAFSGFREQKKPOERNIYVSDZSUOVGXSRRIPWERDIPCFSDIQZIASEJVCGXAYBGYXFPSREKFMEXEBIYDGFKREOWGXEQSXSKXGYRRRVMEKFFIPIWJSKFDJMBGCC".to_string());
		let filtered_dict = filter_dictionary(&get_dictionary("./dictionary.txt"), 8);
		let first_word_dict: Dict = filtered_dict.iter().map(|w| encode(w)).collect();

		b.iter(|| check_attempt(&attempt, &first_word_dict))
	}

	#[bench]
	fn bench_main(b: &mut Bencher) {
		b.iter(|| main())
	}
}

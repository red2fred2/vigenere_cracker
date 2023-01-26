/**
 * Decodes a number 0 to 25 as a character a-z. Returns None when out of range.
 */
fn decode_char(code: &Option<u8>) -> Option<char> {
	match code {
		Some(c) => char::from_digit(u32::from(c + 10), 36),
		None => None
	}
}

/**
 * Decodes a Vec of numbers 0 to 25 and return a lowercase string. Returns None
 * when any numbers are out of range.
 */
fn decode_str(code: &Vec<Option<u8>>) -> Option<String> {
	code.iter().map(|c| decode_char(c)).collect()
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
 * Makes all characters lowercase then strips out non a-z ones.
 */
fn strip_message(message: &String) -> String {
	message.to_lowercase().chars().filter(|c| c >= &'a' && c <= &'z').collect()
}

fn main() {

}

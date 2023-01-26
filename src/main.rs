/**
 * Encodes a unicode as a number 0 to 25 when the character is a-z|A-Z.
 * Returns None when the character is not alphabetical.
 */
fn encode_char(message: char) -> Option<u8> {
	match message.to_digit(36) {
		Some(n) => if n < 10 {
			None
		} else {
			Some(u8::try_from(n).unwrap() - 10)
		},
		None => None
	}
}

fn main() {
	println!("{:?}", encode_char('Z'));
}

fn main() {
	// string slicing
	let s = String::from("hello world");

	let hello = &s[0..5]; // equivalent to [..5]
	let world = &s[6..11]; // equivalent to [6..]

	// with &str, works on variety
	let my_string = String::from("hello world");

	let word = first_word(&my_string[..]); // slices of Strings
	let word = first_word("hello world"); // works on literals
}

// returns first word of string
fn first_word(s: &str) -> &str { // use &str instead of &String to be more general
	let bytes = s.as_bytes();

	// look for first occurence of space
	for (i, &item) in bytes.iter().enumerate() {
		if item = b' ' {
			return &s[0..i];
		}
	}

	&s[..]
}

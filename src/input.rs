use std::io::{self, Write};

pub fn get(message: &str) -> String {
	print!("{}", message);
	io::stdout().flush().unwrap();

	let mut input = String::new();
	io::stdin().read_line(&mut input).expect("Failed to get input");

	if let Some('\n') = input.chars().next_back() {
		input.pop();
	}
	if let Some('\r') = input.chars().next_back() {
		input.pop();
	}

	input
}

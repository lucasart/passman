use std::collections::HashMap;
use std::str::SplitWhitespace;

#[derive(Default)]
struct Data {
	map: HashMap<String, String>
}

impl Data {
	fn show(&self) {
		println!("{:?}", self.map);
	}

	fn insert(&mut self, key: &str, value: &str) {
		// FIXME: ask user to confirm in case of overwrite
		self.map.insert(String::from(key), String::from(value));
	}

	fn remove(&mut self, key: &str) {
		// FIXME: prompt if key didn't exist
		self.map.remove(key);
	}
}

fn parse_insert(tokens: &mut SplitWhitespace<'_>, data: &mut Data) {
	match tokens.next() {
		Some(key) =>
			match tokens.next() {
				Some(value) => data.insert(key, value),
				None => println!("missing value")
			}
		None => println!("expected key")
	}
}

fn parse_remove(tokens: &mut SplitWhitespace<'_>, data: &mut Data) {
	match tokens.next() {
		Some(key) => data.remove(key),
		None => println!("expected key")
	}
}

fn parse_generate(tokens: &mut SplitWhitespace<'_>) {
	match tokens.next() {
		Some(token) =>
			match token.parse::<u8>() {
				Ok(value) => println!("TODO: generate a {} character password", value),
				Err(_) => println!("{} is not a valid number", token)
			}
		None => println!("expected number of characters")
	}
}

fn parse_command<'a>(command: &'a str, tokens: &mut SplitWhitespace<'a>, data: &mut Data) {
	match command {
		"insert" => parse_insert(tokens, data),
		"remove" => parse_remove(tokens, data),
		"generate" => parse_generate(tokens),
		"view" => data.show(),
		_ => println!("unknown command {}", command)
	}
}

fn main() {
	let mut data: Data = Default::default();

	loop {
		let mut line = String::new();
		std::io::stdin().read_line(&mut line).unwrap();
		let mut tokens = line.trim_end().split_whitespace();

		match tokens.next() {
			Some(command) => parse_command(command, &mut tokens, &mut data),
			None => println!("command expected")

		}
	}
}

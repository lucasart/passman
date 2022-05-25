use std::collections::HashMap;
use std::str::SplitWhitespace;
use rand::prelude::*;

#[derive(Default)]
struct Data {
	map: HashMap<String, String>
}

impl Data {
	fn show(&self) {
		println!("{:?}", self.map);
	}

	fn add(&mut self, key: &str, value: &str) {
		match self.map.get(key) {
			Some(_) => println!("entry {} already exists", key),
			None =>
				match self.map.insert(String::from(key), String::from(value)) {
					Some(_) => unreachable!(),
					None => println!("added entry {}", key)
				}
		}
	}

	fn remove(&mut self, key: &str) {
		match self.map.remove(key) {
			Some(_) => println!("removed entry '{}'", key),
			None => println!("could not find entry '{}'", key)
		}
	}
}

fn parse_add(tokens: &mut SplitWhitespace<'_>, data: &mut Data) {
	match tokens.next() {
		Some(key) =>
			match tokens.next() {
				Some(value) => data.add(key, value),
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

fn generate(count: u8) {
	let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+-".as_bytes();
	let mut rng = thread_rng();

	let password: Vec<u8> = (0..count)
		.map(|_| { alphabet[rng.gen_range(0..alphabet.len())] })
		.collect();

	println!("{}", std::str::from_utf8(&password).unwrap());
}

fn parse_generate(tokens: &mut SplitWhitespace<'_>) {
	match tokens.next() {
		Some(token) =>
			match token.parse::<u8>() {
				Ok(value) => generate(value),
				Err(_) => println!("{} is not a valid number", token)
			}
		None => println!("expected number of characters")
	}
}

fn parse_command<'a>(command: &'a str, tokens: &mut SplitWhitespace<'a>, data: &mut Data) {
	match command {
		"add" => parse_add(tokens, data),
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
			Some(command) =>
				if command == "quit" {
					break;
				} else {
					parse_command(command, &mut tokens, &mut data);
				}
			None => println!("command expected")

		}
	}
}

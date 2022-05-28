use std::collections::BTreeMap;
use std::str::SplitWhitespace;
use std::fs::File;
use std::io::Write;
use rand::{RngCore, Rng, rngs::OsRng};
use chacha20poly1305::{XChaCha20Poly1305, Key, XNonce};
use chacha20poly1305::aead::{Aead, NewAead};

#[derive(Default, Debug)]
struct Data {
	map: BTreeMap<String, String>
}

impl Data {
	fn view(&self, prefix: Option<&str>) {
		match prefix {
			Some(prefix) =>
				for entry in self.map.keys().filter(|k| k.starts_with(prefix)) {
					println!("{} {}", entry, self.map[entry]);
				}
			None =>
				for (key, value) in self.map.iter() {
					println!("{} {}", key, value);
				}
		}
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

	fn to_bytes(&self) -> Vec<u8> {
		let mut result = Vec::<u8>::new();

		for (key, value) in self.map.iter() {
			result.extend(key.as_bytes());
			result.push(b' ');
			result.extend(value.as_bytes());
			result.push(b'\n');
		}

		result
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
	let password: Vec<u8> = (0..count)
		.map(|_| OsRng.gen_range(33..126))  // any printable ASCII character
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

fn save(filepath: &str, password: &str, data: &Data) {
	let key = Key::from_slice(b"an example very very secret key."); // FIXME: hash(password) instead
	let cipher = XChaCha20Poly1305::new(key);

	let mut nonce = [0u8; 24];
	OsRng.fill_bytes(&mut nonce);

	let before: Vec<u8> = data.to_bytes();
	let after = cipher.encrypt(&nonce.into(), before.as_ref()).expect("encryption failure!");

	let mut file = File::create(filepath).unwrap();
	file.write(&nonce).unwrap();
	file.write(&after).unwrap();
}

fn parse_save(tokens: &mut SplitWhitespace<'_>, data: &Data) {
	match tokens.next() {
		Some(filepath) => {
			match tokens.next() {
				Some(password) => save(filepath, password, data),
				None => println!("missing key")
			}
		}
		None => println!("missing file")
	}
}

fn parse_command<'a>(command: &'a str, tokens: &mut SplitWhitespace<'a>, data: &mut Data) {
	match command {
		"add" => parse_add(tokens, data),
		"remove" => parse_remove(tokens, data),
		"generate" => parse_generate(tokens),
		"view" => data.view(tokens.next()),
		"save" => parse_save(tokens, data),
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

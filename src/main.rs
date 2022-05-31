use std::collections::BTreeMap;
use std::str::SplitWhitespace;
use std::fs::File;
use std::io::{Read, Write};
use rand::{RngCore, Rng, rngs::OsRng};
use chacha20::{XChaCha20, cipher::{KeyIvInit, StreamCipher}};
use blake2::{Blake2b, Digest};

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

	fn from_bytes(&mut self, bytes: &Vec<u8>) -> Result<(), std::str::Utf8Error> {
		let text = std::str::from_utf8(&bytes[..bytes.len()-1])?;
		self.map.clear();

		for line in text.split('\n') {
			let words: Vec<&str> = line.split(' ').collect();
			assert_eq!(2, words.len());  // FIXME: handle gracefully at runtime
			self.map.insert(words[0].to_owned(), words[1].to_owned());
		}

		Ok(())
	}
}

fn generate(count: u8) {
	let password: Vec<u8> = (0..count)
		.map(|_| OsRng.gen_range(33..126))  // any printable ASCII character
		.collect();

	println!("{}", std::str::from_utf8(&password).unwrap());
}

fn get_password_hash() -> [u8; 32] {
	let password = rpassword::prompt_password("password: ").unwrap();
	let hash: [u8; 32] = Blake2b::digest(password.as_bytes())[..32].try_into().unwrap();
	hash
}

fn save(filepath: &str, hash: [u8; 32], data: &Data) -> std::io::Result<()> {
	let mut nonce = [0u8; 24];
	OsRng.fill_bytes(&mut nonce);

	let mut file = File::create(filepath)?;
	file.write(&nonce)?;

	let mut cipher = XChaCha20::new(&hash.into(), &nonce.into());

	let mut buffer: Vec<u8> = data.to_bytes();
	cipher.apply_keystream(&mut buffer);
	file.write(&buffer)?;

	println!("{} saved successfully", filepath);
	Ok(())
}

fn load(filepath: &str, hash: [u8; 32], data: &mut Data) -> std::io::Result<()> {
	let mut nonce = [0u8; 24];
	let mut file = File::open(filepath)?;
	file.read(&mut nonce)?;

	let mut cipher = XChaCha20::new(&hash.into(), &nonce.into());

	let mut buffer = Vec::<u8>::new();
	file.read_to_end(&mut buffer)?;

	cipher.apply_keystream(&mut buffer);
	match data.from_bytes(&buffer) {
		Ok(_) => println!("{} loaded successfully", filepath),
		Err(err) => println!("wrong password or corrupted file. {}", err)
	}

	Ok(())
}

fn handle_add(tokens: Vec<&str>, data: &mut Data) {
	data.add(tokens[0], tokens[1]);
}

fn handle_remove(tokens: Vec<&str>, data: &mut Data) {
	data.remove(tokens[0]);
}

fn handle_view(tokens: Vec<&str>, data: &mut Data) {
	let prefix = if tokens.len() >= 1 { Some(tokens[0]) } else { None};
	data.view(prefix);
}

fn handle_generate(tokens: Vec<&str>, _: &mut Data) {
	match tokens.len() {
		1 => match tokens[0].parse::<u8>() {
			Ok(value) => generate(value),
			Err(_) => println!("{} is not a valid number", tokens[0])
		}
		0 => generate(10),
		_ => unreachable!()
	}
}

fn handle_save(tokens: Vec<&str>, data: &mut Data) {
	if let Err(io_err) = save(tokens[0], get_password_hash(), data) {
		println!("I/O error. {:?}", io_err);
	}
}

fn handle_load(tokens: Vec<&str>, data: &mut Data) {
	if let Err(io_err) = load(tokens[0], get_password_hash(), data) {
		println!("I/O error. {:?}", io_err);
	}
}

struct Command {
	name: String,
	help: String,
	min_params: usize,
	max_params: usize,
	handler: fn(Vec<&str>, &mut Data) -> ()
}

fn main() {
	let commands = [
		Command {name: "add".to_owned(), help: "add key value".to_owned(), min_params: 2,
			max_params: 2, handler: handle_add},
		Command {name: "remove".to_owned(), help: "remove key".to_owned(), min_params: 1,
			max_params: 1, handler: handle_remove},
		Command {name: "view".to_owned(), help: "view [key]".to_owned(), min_params: 0,
			max_params: 1, handler: handle_view},
		Command {name: "save".to_owned(), help: "save file".to_owned(), min_params: 1,
			max_params: 1, handler: handle_save},
		Command {name: "load".to_owned(), help: "load file".to_owned(), min_params: 1,
			max_params: 1, handler: handle_load},
		Command {name: "generate".to_owned(), help: "generate [length]".to_owned(), min_params: 0,
			max_params: 1, handler: handle_generate},
	];

	let mut data: Data = Default::default();

	loop {
		let mut line = String::new();
		std::io::stdin().read_line(&mut line).unwrap();
		let mut tokens = line.trim_end().split_whitespace();

		match tokens.next() {
			Some(name) => match name {
				"help" => commands
					.iter()
					.map(|c| println!("{}", c.help))
					.collect(),
				"quit" => break,
				_ => {
					let found: Vec<&Command> = commands
						.iter()
						.filter(|c| c.name == name)
						.collect();

					if found.len() == 1 {
						let command = found[0];
						let params: Vec<&str> = tokens.collect();

						if params.len() < command.min_params {
							println!("{} expects at least {} parameters", command.name,
								command.min_params);
						} else if params.len() > command.max_params {
							println!("{} expects at most {} parameters", command.name,
								command.max_params);
						} else {
							(command.handler)(params, &mut data);
						}
				    } else {
						assert_eq!(0, found.len());
						println!("command not found {}", name);
					}
				}
			},
			None => println!("command expected")
		}
	}
}

use blake2::{Blake2b, Digest};
use chacha20::{XChaCha20, cipher::{KeyIvInit, StreamCipher}};
use rand::{RngCore, rngs::OsRng};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{Read, Write};

#[derive(Default, Debug)]
pub struct Data {
	map: BTreeMap<String, String>
}

impl Data {
	pub fn view(&self, prefix: Option<&str>) {
		match prefix {
			Some(prefix) => self.map.keys()
				.filter(|key| key.starts_with(prefix))
				.for_each(|key| println!("{}\t{}", key, self.map[key])),
			None => self.map.iter()
				.for_each(|(key, value)| println!("{}\t{}", key, value)),
		}
	}

	pub fn add(&mut self, key: &str, value: &str) {
		match self.map.get(key) {
			Some(_) => println!("entry {} already exists", key),
			None =>
				match self.map.insert(String::from(key), String::from(value)) {
					Some(_) => unreachable!(),
					None => println!("added entry {}", key)
				}
		}
	}

	pub fn remove(&mut self, key: &str) {
		match self.map.remove(key) {
			Some(_) => println!("removed entry '{}'", key),
			None => println!("could not find entry '{}'", key)
		}
	}

	pub fn to_bytes(&self) -> Vec<u8> {
		let mut result = Vec::<u8>::new();

		for (key, value) in self.map.iter() {
			result.extend(key.as_bytes());
			result.push(b'\t');
			result.extend(value.as_bytes());
			result.push(b'\n');
		}

		result
	}

	pub fn from_bytes(&mut self, bytes: &Vec<u8>) -> Result<(), std::str::Utf8Error> {
		let text = std::str::from_utf8(&bytes[..bytes.len()-1])?;
		self.map.clear();

		for line in text.split('\n') {
			let words: Vec<&str> = line.split('\t').collect();
			assert_eq!(2, words.len());  // FIXME: handle gracefully at runtime
			self.map.insert(words[0].to_owned(), words[1].to_owned());
		}

		Ok(())
	}

	pub fn save(&self, filepath: &str, password: &str) -> std::io::Result<()> {
		let mut nonce = [0u8; 24];
		OsRng.fill_bytes(&mut nonce);

		let mut file = File::create(filepath)?;
		file.write(&nonce)?;
		let mut cipher = XChaCha20::new(Blake2b::digest(password.as_bytes())[..32].as_ref().into(),
			&nonce.into());

		let mut buffer: Vec<u8> = self.to_bytes();
		cipher.apply_keystream(&mut buffer);
		file.write(&buffer)?;

		println!("{} saved successfully", filepath);
		Ok(())
	}

	pub fn load(&mut self, filepath: &str, password: &str) -> std::io::Result<()> {
		let mut nonce = [0u8; 24];
		let mut file = File::open(filepath)?;
		file.read(&mut nonce)?;

		let mut cipher = XChaCha20::new(Blake2b::digest(password.as_bytes())[..32].as_ref().into(),
			&nonce.into());

		let mut buffer = Vec::<u8>::new();
		file.read_to_end(&mut buffer)?;

		cipher.apply_keystream(&mut buffer);
		match self.from_bytes(&buffer) {
			Ok(_) => println!("{} loaded successfully", filepath),
			Err(err) => println!("wrong password or corrupted file. {}", err)
		}

		Ok(())
	}
}

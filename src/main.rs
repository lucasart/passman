mod data;
use crate::data::Data;
use std::collections::BTreeMap;

use rand::{Rng, rngs::OsRng};

fn generate(count: u8) {
	let password: Vec<u8> = (0..count)
		.map(|_| OsRng.gen_range(33..126))  // printable ASCII character
		.collect();

	println!("{}", std::str::from_utf8(&password).unwrap());
}

fn handle_add(params: Vec<&str>, data: &mut Data) {
	data.add(params[0], params[1]);
}

fn handle_remove(params: Vec<&str>, data: &mut Data) {
	data.remove(params[0]);
}

fn handle_list(params: Vec<&str>, data: &Data) {
	let prefix = if !params.is_empty() { Some(params[0]) } else { None};
	data.list(prefix);
}

fn handle_view(params: Vec<&str>, data: &Data) {
	data.view(params[0]);
}

fn handle_generate(params: Vec<&str>) {
	match params.first() {
		Some(token) => match token.parse::<u8>() {
			Ok(value) => generate(value),
			Err(_) => println!("{token} is not a valid number"),
		},
		None => generate(10),
	}
}

fn handle_save(params: Vec<&str>, data: &Data) {
	let pwd = rpassword::prompt_password("password: ").unwrap();
	let pwd_conf = rpassword::prompt_password("confirm password: ").unwrap();

	if pwd_conf == pwd {
		if let Err(io_err) = data.save(params[0], &pwd) {
			println!("I/O error. {:?}", io_err);
		}
	} else {
		println!("password confirmation failed. save aborted.");
	}
}

fn handle_load(params: Vec<&str>, data: &mut Data) {
	if let Err(io_err) = data.load(params[0], &rpassword::prompt_password("password: ").unwrap()) {
		println!("I/O error. {:?}", io_err);
	}
}

// Types of handler_*() function pointers
enum Handler {
	ND(fn(Vec<&str>) -> ()),  // no data
	ID(fn(Vec<&str>, &Data) -> ()),  // immutable data
	MD(fn(Vec<&str>, &mut Data) -> ()),  // mutable data
}

struct Command {
	help: &'static str,
	param_cnt: (usize, usize),
	handler: Handler,
}

fn handle_command(name: &str, params: Vec<&str>, commands: &BTreeMap<&str, Command>, data: &mut Data) {
	match commands.get(name) {
		Some(command) => {
			if params.len() < command.param_cnt.0 {
				println!("{} expects at least {} parameters", name, command.param_cnt.0);
			} else if params.len() > command.param_cnt.1 {
				println!("{} expects at most {} parameters", name, command.param_cnt.1);
			} else {
				match command.handler {
					Handler::ND(h) => h(params),
					Handler::ID(h) => h(params, data),
					Handler::MD(h) => h(params, data),
				}
			}
		}
		None => println!("command not found {}", name)
	}
}

fn main() {
	let commands = BTreeMap::from([
		("add", Command {help: "key\tvalue", param_cnt: (2, 2), handler: Handler::MD(handle_add)}),
		("remove", Command {help: "key", param_cnt: (1, 1), handler: Handler::MD(handle_remove)}),
		("list", Command {help: "[prefix]", param_cnt: (0, 1), handler: Handler::ID(handle_list)}),
		("view", Command {help: "key", param_cnt: (1, 1), handler: Handler::ID(handle_view)}),
		("save", Command {help: "file", param_cnt: (1, 1), handler: Handler::ID(handle_save)}),
		("load", Command {help: "file", param_cnt: (1, 1), handler: Handler::MD(handle_load)}),
		("gen", Command {help: "[length]", param_cnt: (0, 1), handler: Handler::ND(handle_generate)}),
	]);

	let mut data: Data = Default::default();

	loop {
		let mut line = String::new();
		std::io::stdin().read_line(&mut line).unwrap();
		let mut tokens = line.trim_end().split('\t');

		match tokens.next() {
			Some(name) => match name {
				"quit" => break,
				"help" => commands.iter()
					.for_each(|(name, command)| println!("{}\t{}", name, command.help)),
				_ => handle_command(name, tokens.collect(), &commands, &mut data),
			},
			None => println!("command expected")
		}
	}
}

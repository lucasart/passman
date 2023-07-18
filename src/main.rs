mod data;
use crate::data::Data;

use rand::{Rng, rngs::OsRng};

fn generate(count: u8) {
	let password: Vec<u8> = (0..count)
		.map(|_| OsRng.gen_range(32..126))  // printable ASCII character
		.collect();

	println!("{}", std::str::from_utf8(&password).unwrap());
}

fn handle_add(params: Vec<&str>, data: &mut Data) {
	data.add(params[0], params[1]);
}

fn handle_remove(params: Vec<&str>, data: &mut Data) {
	data.remove(params[0]);
}

fn handle_view(params: Vec<&str>, data: &Data) {
	let prefix = if params.len() >= 1 { Some(params[0]) } else { None};
	data.view(prefix);
}

fn handle_generate(params: Vec<&str>) {
	match params.len() {
		1 => match params[0].parse::<u8>() {
			Ok(value) => generate(value),
			Err(_) => println!("{} is not a valid number", params[0])
		}
		0 => generate(10),
		_ => unreachable!()
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

fn handle_help(params: Vec<&str>) {
	if params.len() > 0 {
		COMMANDS.iter()
			.filter(|c| c.name.starts_with(params[0]))
			.for_each(|c| println!("{}", c.help));
	} else {
		COMMANDS.iter()
			.for_each(|c| println!("{}", c.help));
	}
}

// Types of handler_*() function pointers
enum Handler {
	ND(fn(Vec<&str>) -> ()),  // no data
	ID(fn(Vec<&str>, &Data) -> ()),  // immutable data
	MD(fn(Vec<&str>, &mut Data) -> ()),  // mutable data
}

struct Command {
	name: &'static str,
	help: &'static str,
	min_params: usize,
	max_params: usize,
	handler: Handler,
}

const COMMANDS: [Command; 7] = [
	Command {name: "add", help: "add\tkey\tvalue", min_params: 2, max_params: 2,
		handler: Handler::MD(handle_add)},
	Command {name: "remove", help: "remove\tkey", min_params: 1, max_params: 1,
		handler: Handler::MD(handle_remove)},
	Command {name: "view", help: "view\t[key]", min_params: 0, max_params: 1,
		handler: Handler::ID(handle_view)},
	Command {name: "save", help: "save\tfile", min_params: 1, max_params: 1,
		handler: Handler::ID(handle_save)},
	Command {name: "load", help: "load\tfile", min_params: 1, max_params: 1,
		handler: Handler::MD(handle_load)},
	Command {name: "gen", help: "gen\t[length]", min_params: 0, max_params: 1,
		handler: Handler::ND(handle_generate)},
	Command {name: "help", help: "help\t[command]", min_params: 0, max_params: 1,
		handler: Handler::ND(handle_help)},
];

fn handle_command(name: & str, params: Vec<&str>, data: &mut Data) {
	let found: Vec<&Command> = COMMANDS.iter()
		.filter(|c| c.name == name)
		.collect();

	if found.len() == 1 {
		let command = found[0];

		if params.len() < command.min_params {
			println!("{} expects at least {} parameters", command.name, command.min_params);
		} else if params.len() > command.max_params {
			println!("{} expects at most {} parameters", command.name, command.max_params);
		} else {
			match command.handler {
				Handler::ND(h) => h(params),
				Handler::ID(h) => h(params, data),
				Handler::MD(h) => h(params, data),
			}
		}
	} else {
		assert_eq!(0, found.len());
		println!("command not found {}", name);
	}
}

fn main() {
	let mut data: Data = Default::default();

	loop {
		let mut line = String::new();
		std::io::stdin().read_line(&mut line).unwrap();
		let mut tokens = line.trim_end().split('\t');

		match tokens.next() {
			Some(name) => match name {
				"quit" => break,
				_ => handle_command(name, tokens.collect(), &mut data),
			},
			None => println!("command expected")
		}
	}
}

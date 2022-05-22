fn view(_map: &std::collections::HashMap::<String, String>) {
	println!("TODO: view");
}

fn add(_map: &mut std::collections::HashMap::<String, String>, tokens: &mut std::str::SplitWhitespace<'_>) {
	match tokens.next() {
		Some(key) => println!("TODO: add {}", key),
		None => println!("expected key")
	}
}

fn delete(_map: &mut std::collections::HashMap::<String, String>, tokens: &mut std::str::SplitWhitespace<'_>) {
	match tokens.next() {
		Some(key) => println!("TODO: delete {}", key),
		None => println!("expected key")
	}
}

fn generate(tokens: &mut std::str::SplitWhitespace<'_>) {
	match tokens.next() {
		Some(token) =>
			match token.parse::<u8>() {
				Ok(value) => println!("TODO: generate a {} character password", value),
				Err(_) => println!("{} is not a valid number", token)
			}
		None => println!("expected number of characters")
	}
}

fn handle<'a>(map: &mut std::collections::HashMap::<String, String>, command: &'a str, tokens: &mut std::str::SplitWhitespace<'a>) {
	match command {
		"add" => add(map, tokens),
		"delete" => delete(map, tokens),
		"generate" => generate(tokens),
		"view" => view(map),
		_ => println!("unknown command {}", command)
	}
}

fn main() {
	let mut map = std::collections::HashMap::<String, String>::new();

	loop {
		let mut line = String::new();
		std::io::stdin().read_line(&mut line).unwrap();
		let mut tokens = line.trim_end().split_whitespace();

		match tokens.next() {
			Some(command) => handle(&mut map, command, &mut tokens),
			None => println!("command expected")

		}	
	}
}

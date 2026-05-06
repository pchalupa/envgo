pub fn info(message: &str) {
	println!("\x1b[90m{}\x1b[0m", message);
}

pub fn error(message: &str) {
	eprintln!("\x1b[31m{}\x1b[0m", message);
}
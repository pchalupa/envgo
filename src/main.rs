use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::Command;

const ENV_FILES: &[&str] = &[".env"];

mod logger {
    pub fn info(message: &str) {
        println!("\x1b[90m{}\x1b[0m", message);
    }

    pub fn error(message: &str) {
        eprintln!("\x1b[31m{}\x1b[0m", message);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = args.get(1).unwrap_or_else(|| {
        logger::error("Usage: envgo <command> [args...]");
        std::process::exit(1)
    });
    let command_args = &args[2..];

    let path = Path::new(ENV_FILES[0]);
    let file = match File::open(path) {
        Err(_) => {
            logger::error("Could not find .env file!");
            std::process::exit(1);
        }
        Ok(file) => file,
    };
    let reader = BufReader::new(file);

    let mut envs: HashMap<String, String> = HashMap::new();

    for line in reader.lines() {
        let input_line = match line {
            Err(_) => {
                logger::error("Unable to read line!");
                std::process::exit(1)
            }
            Ok(it) => it,
        };

        if let Some((key, value)) = input_line.split_once("=") {
            let key = key.trim();
            let value = value.trim();
            envs.insert(key.to_string(), value.to_string());
        }
    }

    let keys = envs.keys().map(|key| key.as_str()).collect::<Vec<_>>().join(", ");
    let files = ENV_FILES.join(", ");

    logger::info(&format!("Environment variables loaded from the {}: {}", files, keys));

    Command::new(command)
        .args(command_args)
        .envs(&envs)
        .status()
        .unwrap_or_else(|e| {
            eprintln!("Failed to execute command: {}", e);
            std::process::exit(1);
        });
}
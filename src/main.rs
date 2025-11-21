use std::env;
use std::fs::File;
use std::path::Path;
use std::process::Command;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};

const ENV_FILES: &[&str] = &[".env"];

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];
    let command_args = &args[2..];

    let path = Path::new(ENV_FILES[0]);
    let file = match File::open(path) {
        Err(_) => {
            eprintln!("Could not open the file!");
            std::process::exit(1);
        },
        Ok(file) => file
    };
    let reader = BufReader::new(file);

    let mut envs: HashMap<String, String> = HashMap::new();

    for line in reader.lines() {
        let input_line = match line {
            Err(_) => {
                eprintln!("Unable to read line!");
                std::process::exit(1)
            },
            Ok(it) => it
        };

        if let Some((key, value)) = input_line.split_once("=") {
            envs.insert(key.trim().to_string(), value.trim().to_string());
        }
    }

    Command::new(command).args(command_args).envs(&envs).status().unwrap_or_else(|e| {
        eprintln!("Failed to execute command: {}", e);
        std::process::exit(1);
    });
}

#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "exit 0" {
            process::exit(0)
        }
        if input.is_empty() {
            continue;
        }

        let parts: Vec<&str> = input.split_whitespace().collect();
        if !parts.is_empty() && parts[0] == "echo" {
            let message = parts[1..].join(" ");
            println!("{}", message);
            continue;
        }
        println!("{}: command not found", input)
        }
}

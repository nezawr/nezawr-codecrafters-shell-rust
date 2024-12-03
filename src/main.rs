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
        println!("{}: command not found", input)
        }
}

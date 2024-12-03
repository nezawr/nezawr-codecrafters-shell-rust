#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;

fn main() {
    // Define built-in commands
    let built_in: &[&str] = &["type", "echo", "exit"];
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        if input == "exit 0" {
            process::exit(0)
        }


        let parts: Vec<&str> = input.split_whitespace().collect();
        let command = parts[0];
        let args = &parts[1..];

        // Match and handle commands
        match command {
            "type" => handle_type(args, built_in),
            "echo" => handle_echo(args),
            _ => println!("{}: command not found", command)
        }
    }
}

fn handle_type(args: &[&str], built_in: &[&str]) {
    if args.is_empty() {
        println!("type: missing argument");
        return;
    }
    let target = args.join(" ");
    if built_in.contains(&target.as_str()) {
        println!("{} is a shell builtin", target);
    } else {
        println!("{}: not found", target);
    }
}

fn handle_echo(args: &[&str]) {
    println!("{}", args.join(" "));
}

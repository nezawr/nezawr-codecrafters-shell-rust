#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::{self, Command};
use std::env;
use std::path::Path;

fn main() {
    // Define built-in commands
    let built_in: &[&str] = &["type", "echo", "exit", "pwd"];
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
            "pwd"  => handle_pwd(),
            _ => handle_external(command, args),
        }
    }
}

fn handle_pwd() {
    match env::current_dir() {
        Ok(path) => println!("{}", path.display()),
        Err(err) => eprintln!("pwd: error: {}", err),
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
        return
    }
    let path = env::var("PATH").unwrap_or_default();
    for dir in path.split(":") {
        let candidate = format!("{}/{}", dir, target);
        if Path::new(&candidate).is_file() {
            println!("{} is {}", target, candidate);
            return;
        }
    }
    println!("{}: not found", target);
}

fn handle_echo(args: &[&str]) {
    println!("{}", args.join(" "));
}

fn handle_external(command: &str, args: &[&str]) {
    let path = env::var("PATH").unwrap_or_default();
    for dir in path.split(":") {
        let candidate = format!("{}/{}", dir, command);
        if Path::new(&candidate).is_file() {
            // Execute the command
            match Command::new(&candidate).args(args).output() {
                Ok(output) => {
                    print!("{}", String::from_utf8_lossy(&output.stdout));
                }
                Err(err) => {
                    eprintln!("Error executing {}: {}", command, err);
                }
            }
            return;
        }
    }
    println!("{}: command not found", command);
}
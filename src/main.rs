#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::{self, Command};
use std::env;
use std::path::Path;

fn main() {
    // Define built-in commands
    let built_in: &[&str] = &["type", "echo", "exit", "pwd", "ce"];
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


        let parts: Vec<String> = parse_arguments(input);
        let command = parts[0].as_str();
        let args: Vec<&str> = parts[1..].iter().map(String::as_str).collect();


        // Match and handle commands
        match command {
            "type" => handle_type(&args, built_in),
            "echo" => handle_echo(&args),
            "pwd"  => handle_pwd(),
            "cd"   => handle_cd(&args),
            _ => handle_external(command, &args),
        }
    }
}

fn parse_arguments(input: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut current = String::new();
    let mut in_single_quotes = false;
    let mut in_double_quotes = false;
    let mut escape_next = false;

    for c in input.chars() {
        if escape_next {
            current.push(c);
            escape_next = false;
            continue;
        }
        match c {
            '\\' if !in_single_quotes && !in_double_quotes => {
                // Escape next character inside double quotes
                escape_next = true;
            }
            '"' if in_double_quotes => {
                // End double-quoted argument
                in_double_quotes = false;
                args.push(current);
                current = String::new();
            }
            '"' if !in_single_quotes => {
                // Start double-quoted argument
                in_double_quotes = true;
            }
            '\'' if in_single_quotes => {
                // End single-quoted argument
                in_single_quotes = false;
                args.push(current);
                current = String::new();
            }
            '\'' if !in_double_quotes => {
                // Start single-quoted argument
                in_single_quotes = true;
            }
            ' ' if !in_single_quotes && !in_double_quotes => {
                // Space outside quotes ends the current argument
                if !current.is_empty() {
                    args.push(current);
                    current = String::new();
                }
            }
            _ => {
                // Add character to the current argument
                current.push(c);
            }
        }
    }
    // Push the last argument if there's any
    if !current.is_empty() {
        args.push(current);
    }
    args
}

fn handle_cd(args: &[&str]) {
    if args.is_empty() {
        eprintln!("cd: missing argument");
        return;
    }
    let mut target = args[0].to_string();
    if target == "~" || target.starts_with("~/") {
        if let Ok(home) = env::var("HOME") {
            target = target.replacen("~", &home, 1);
        } else {
            eprintln!("cd: HOME environment variable not set");
            return;
        }
    }
    if let Err(_) = env::set_current_dir(&target) {
        eprintln!("cd: {}: No such file or directory", target)
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
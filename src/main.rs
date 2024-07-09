use std::{env, fs, io::{self, BufRead, Write}, path::Path, process, str};

mod token;
mod scanner;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => run_prompt(),
        2 => run_file(&args[1]),
        _ => {
            eprintln!("Usage: jlox [script]");
            process::exit(64);
        }
    }
}

fn run_file(path: &str) {
    let bytes = fs::read(Path::new(path)).expect("Failed to read file");
    let content = str::from_utf8(&bytes).expect("Failed to convert bytes");
    run(content);
}

fn run_prompt() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        let bytes_read = reader.read_line(&mut line);
        
        match bytes_read {
            Ok(len) => {
                if len == 0 {
                    break;
                }

                run(&line.trim());
            },
            Err(_) => {
                break;
            }
        }
    }
}

fn run(source: &str) {
    println!("source: {}", source);
    // TODO: Create Scanner
    // TODO: Create Tokens

    // TODO: Print Tokens
}
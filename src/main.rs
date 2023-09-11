use std::env::args;
use std::io::{self, stdout, BufRead, Write};

mod error_handling;

mod token_type;

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() > 1 {
        println!("Usage: jlox [script]");
        std::process::exit(64)
    } else if args.len() == 2 {
        run_file(&args[1]).expect("unable to run file")
    } else {
        run_prompt()
    }
}

fn run_file(path: &String) -> io::Result<()> {
    let buffer: String = std::fs::read_to_string(path)?;
    match run(&buffer) {
        Ok(_) => {}
        Err(lox_error) => {
            lox_error.report("".to_string());
            std::process::exit(65)
        }
    }
    Ok(())
}

fn run_prompt() {
    let stdin = io::stdin();
    print!("> ");

    stdout().flush();

    for line in stdin.lock().lines().flatten() {
        if line.is_empty() {
            break;
        };
        match run(line.as_str()) {
            Ok(_) => {}
            Err(lox_error) => {
                lox_error.report("".to_string());
            }
        }
    }
}

fn run(_source: &str) -> Result<(), error_handling::LoxError> {
    // let mut lexer = Lexer::new(source);
    // let tokens = lexer.eval_tokens()?;

    // for token in tokens {
    //   println!("{:?}", token)
    // }

    Ok(())
}

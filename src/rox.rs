use std::{
    fs::read_to_string,
    io::{stdin, stdout, Write},
};

use crate::scanner::Scanner;

pub struct Rox {
    had_error: bool,
}

impl Rox {
    pub fn new() -> Self {
        Rox { had_error: false }
    }

    pub fn run_repl(&mut self) {
        loop {
            let mut line = String::from("");
            print!("ROX >>> ");
            stdout().flush().expect("could not flush");
            stdin().read_line(&mut line).unwrap_or_else(|err| {
                println!("Error: {}", err);
                std::process::exit(64);
            });
            let line = line.trim();
            self.run(line);
            self.had_error = false;
        }
    }

    pub fn run_source(&mut self, source_path: &str) {
        let source = read_to_string(source_path).unwrap_or_else(|err| {
            eprintln!("Error: {}", err);
            std::process::exit(64);
        });

        self.run(&source);

        if self.had_error {
            std::process::exit(65)
        };
    }
}

impl Rox {
    fn run(&mut self, source: &str) {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        for token in tokens {
            token.print();
        }
        let errors = scanner.errors;
        if errors.len() > 0 {
            self.had_error = true;
            for error in errors {
                error.print();
            }
        }
    }
}

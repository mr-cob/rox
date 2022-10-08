use std::{
    fs::read_to_string,
    io::{stdin, stdout, Write},
};

use crate::{scanner::Scanner, token_type::TokenType};

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

    pub fn error(&mut self, line: usize, message: &str) {
        self.report(line, "", message);
    }
}

impl Rox {
    fn run(&mut self, source: &str) {
        let mut scanner = Scanner::new(source);
        for token in scanner.scan_tokens() {
            if token.token_type == TokenType::Invalid {
                self.error(token.line, "Unexpected Token");
                self.had_error = true;
            }
            token.print();
        }
    }

    fn report(&mut self, line: usize, plece: &str, message: &str) {
        eprintln!("[line {}] Error {}: {}", line, plece, message);
        self.had_error = true;
    }
}

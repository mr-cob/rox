use std::env::args;

use rox::Rox;

mod errors;
mod rox;
mod scanner;
mod token;
mod token_type;

fn main() {
    let args: Vec<String> = args().collect();
    let mut rox_interpreter = Rox::new();

    if args.len() > 3 {
        eprintln!("Usage: rox <source_file's_path>");
        std::process::exit(1);
    } else if args.len() == 2 {
        rox_interpreter.run_source(&args[1]);
    } else {
        rox_interpreter.run_repl();
    }
}

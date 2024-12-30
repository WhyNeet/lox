use std::{env, process};

use lexer::scanner::Scanner;
use parser::Parser;
use runtime::Runtime;

fn main() {
    let input = env::args().nth(1).unwrap();

    let scanner = Scanner::new(input);
    scanner.scan_tokens().unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1)
    });

    let tokens = scanner.tokens();

    let parser = Parser::new(tokens);
    let tree = parser.run().unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1)
    });

    println!("\n--- running ---\n");

    Runtime::new().run(tree).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });
}

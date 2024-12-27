use std::{env, process};

use lexer::scanner::Scanner;

fn main() {
    let input = env::args().nth(1).unwrap();

    let scanner = Scanner::new(input);
    scanner.scan_tokens().unwrap_or_else(|err| {
        println!("{err}");
        process::exit(1)
    });

    println!("tokens: {:?}", scanner.tokens());
}

use std::env;

use lexer::scanner::Scanner;

fn main() {
    let input = env::args().nth(1).unwrap();

    let scanner = Scanner::new(input);
    scanner.scan_tokens();

    println!("tokens: {:?}", scanner.tokens());
}

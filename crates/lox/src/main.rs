pub mod args;
pub mod error;

use std::{fs, process};

use args::Args;
use clap::Parser as _;
use colored::Colorize;
use error::CliError;
use lexer::scanner::Scanner;
use parser::Parser;
use runtime::Runtime;

fn main() {
    let args = Args::parse();

    let input = args
        .file
        .get(0)
        .map(|path| fs::read_to_string(path).unwrap())
        .unwrap_or_else(|| {
            match args.command.unwrap_or_else(|| {
                eprintln!("{}", CliError::MissingFilename.to_string().red());
                process::exit(1);
            }) {
                args::Commands::Eval { code } => code,
            }
        });

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

    Runtime::new().run(tree).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });
}

use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Lox")]
#[command(version, about = "A Rust-based Lox compiler.")]
pub struct Args {
    pub file: Vec<PathBuf>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Eval { code: String },
}

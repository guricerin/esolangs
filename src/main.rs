use std::{fs, path::PathBuf};

use anyhow::Result;
use clap::Clap;

mod ast;
mod interpreter;
mod parser;
mod token;

#[derive(Debug, Clap)]
#[clap(name = env!("CARGO_BIN_NAME"),version=env!("CARGO_PKG_VERSION"),author=env!("CARGO_PKG_AUTHORS"))]
struct Opts {
    #[clap(name = "Bolic code file path")]
    src_path: PathBuf,
}

fn main() -> Result<()> {
    let opts = Opts::parse();
    let code = fs::read_to_string(opts.src_path)?;
    let mut interpreter = interpreter::Interpreter::new();
    interpreter.run(&code)?;

    Ok(())
}

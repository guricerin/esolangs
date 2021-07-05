use std::{fs, path::PathBuf};

use anyhow::Result;
use clap::Clap;

use crate::{compiler::Compiler, vm::VM};

mod compiler;
mod instruction;
mod token;
mod vm;

#[derive(Debug, Clap)]
#[clap(name = env!("CARGO_BIN_NAME"),version=env!("CARGO_PKG_VERSION"),author=env!("CARGO_PKG_AUTHORS"))]
struct Opts {
    #[clap(name = "Whitespace code file path")]
    src_path: PathBuf,
}

fn main() -> Result<()> {
    let opts = Opts::parse();
    let code = fs::read_to_string(opts.src_path)?;
    let insts = Compiler::new(code).compile()?;
    VM::new(insts).run()?;

    Ok(())
}

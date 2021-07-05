use std::{fs, path::PathBuf};

use anyhow::{anyhow, Result};
use clap::Clap;

use crate::{compiler::Compiler, instruction::Instruction, vm::VM};

mod compiler;
mod instruction;
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
    let insts = vec![
        Instruction::Push(0x41),
        Instruction::CharOut,
        Instruction::Exit,
    ];
    VM::new(insts).run()?;

    Ok(())
}

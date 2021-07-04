use clap::Clap;
use std::error::Error;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::result::Result;

mod interpreter;

#[derive(Clap, Debug)]
#[clap(name = env!("CARGO_BIN_NAME"),version=env!("CARGO_PKG_VERSION"),author=env!("CARGO_PKG_AUTHORS"))]
struct Opts {
    #[clap(name = "HQ9+ code file path")]
    src_path: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts = Opts::parse();
    let code = fs::read_to_string(opts.src_path)?;
    let stdout = io::stdout();
    let mut interpreter = interpreter::Interpreter::new(stdout, code);
    interpreter.run()?;
    // println!("{}", &code);
    Ok(())
}

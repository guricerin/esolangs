use std::error::Error;
use std::io::{self, BufWriter, Write};
use std::result::Result;

pub struct Interpreter<W: io::Write> {
    writer: BufWriter<W>,
    src_code: String,
    count: u64,
}

impl<W: io::Write> Interpreter<W> {
    pub fn new(writer: W, code: String) -> Self {
        let writer = BufWriter::new(writer);
        Self {
            writer: writer,
            src_code: code,
            count: 0,
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        self.print_source()?;
        Ok(())
    }

    fn print_hello(&mut self) -> Result<(), Box<dyn Error>> {
        self.writer.write(b"Hello, world!\n")?;
        Ok(())
    }

    fn print_source(&mut self) -> Result<(), Box<dyn Error>> {
        self.writer.write(self.src_code.as_bytes())?;
        Ok(())
    }

    fn print_99_bottles_of_beer(&mut self) -> Result<(), Box<dyn Error>> {
        todo!();
        Ok(())
    }
}

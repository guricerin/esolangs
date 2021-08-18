use std::error::Error;
use std::io::{self, BufWriter, Write};
use std::result::Result;

use crate::token::{self, Token};

#[derive(Debug)]
pub struct Interpreter<W: io::Write> {
    writer: BufWriter<W>,
    src_code: String,
    count: u64,
}

impl<W: io::Write> Interpreter<W> {
    pub fn new(output: W, code: String) -> Self {
        let writer = BufWriter::new(output);
        Self {
            writer: writer,
            src_code: code,
            count: 0,
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let tokens = token::tokenize(&self.src_code);
        for tok in tokens.iter() {
            match tok {
                Token::Hello => self.print_hello()?,
                Token::Quine => self.print_source()?,
                Token::Ninety => self.print_99_bottles_of_beer()?,
                Token::Plus => self.increment(),
                Token::Ignore => (),
            }
        }
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

    fn head_to_upper(s: &str) -> String {
        let mut c = s.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().chain(c).collect(),
        }
    }

    fn print_99_bottles_of_beer(&mut self) -> Result<(), Box<dyn Error>> {
        // [0, 100)
        for k in (0..100).rev() {
            let (before, after) = match k {
                0 => ("no more bottles".to_owned(), "99 bottles".to_owned()),
                1 => ("1 bottle".to_owned(), "no more bottles".to_owned()),
                2 => ("2 bottles".to_owned(), "1 bottle".to_owned()),
                _ => (format!("{} bottles", k), format!("{} bottles", k - 1)),
            };

            let action = match k {
                0 => "Go to the store and buy some more",
                _ => "Take one down and pass it around",
            };

            let buf = format!(
                r#"{} of beer on the wall, {} of beer.
{}, {} of beer on the wall.
"#,
                Self::head_to_upper(&before),
                before,
                action,
                after
            );

            self.writer.write(buf.as_bytes())?;
        }
        Ok(())
    }

    fn increment(&mut self) {
        self.count += 1;
    }
}

use std::io::{self, BufWriter, Write};

use anyhow::Result;

use crate::{ast::*, parser, token};

#[derive(Debug)]
pub struct Interpreter {
    ret_code: i64,
}

impl Interpreter {
    pub fn new() -> Self {
        Self { ret_code: 0 }
    }

    pub fn ret_code(&self) -> i64 {
        self.ret_code
    }

    pub fn run(&mut self, code: &str) -> Result<()> {
        let tokens = token::lex(code)?;
        let ast = parser::parse(tokens)?;
        self.eval(&ast)?;
        Ok(())
    }

    fn eval(&mut self, ast: &Ast) -> Result<()> {
        match ast {
            Ast::Stmts(stmts) => {
                for stmt in stmts.iter() {
                    self.e_stmt(stmt)?;
                }
            }
        };
        Ok(())
    }

    fn e_stmt(&self, stmt: &Stmt) -> Result<()> {
        match stmt {
            Stmt::NumOut(expr) => {
                let x = self.e_expr(expr)?.to_string();
                let mut writer = BufWriter::new(io::stdout());
                writer.write(x.as_bytes())?;
                writer.flush()?;
            }
            Stmt::CharOut(expr) => {
                let x = self.e_expr(expr)?;
                let mut writer = BufWriter::new(io::stdout());
                // ASCIIコードとみなす
                writer.write(&[x as u8])?;
                writer.flush()?;
            }
        };
        Ok(())
    }

    fn e_expr(&self, ast: &Expr) -> Result<i64> {
        let res = match ast {
            Expr::Int(i) => *i,
            Expr::BinOp { op, l, r } => match op {
                BinOp::Add => {
                    let l = self.e_expr(l)?;
                    let r = self.e_expr(r)?;
                    l + r
                }
                BinOp::Sub => {
                    let l = self.e_expr(l)?;
                    let r = self.e_expr(r)?;
                    l - r
                }
                BinOp::Mul => {
                    let l = self.e_expr(l)?;
                    let r = self.e_expr(r)?;
                    l * r
                }
                BinOp::Div => {
                    let l = self.e_expr(l)?;
                    let r = self.e_expr(r)?;
                    l / r
                }
            },
        };

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::Interpreter;

    #[test]
    fn add() {
        let code = "①＋②";
        let mut interpreter = Interpreter::new();
        interpreter.run(code).unwrap();
        let expect = 3;
        assert_eq!(expect, interpreter.ret_code);
    }
}

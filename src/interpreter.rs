use anyhow::Result;

use crate::{ast::*, parser, token};

#[derive(Debug)]
pub struct Interpreter {
    ast: Ast,
}

impl Interpreter {
    pub fn new(code: &str) -> Result<Self> {
        let tokens = token::lex(code)?;
        let ast = parser::Parser::new(tokens).parse()?;
        let res = Self { ast: ast };
        Ok(res)
    }

    pub fn run(&self) -> Result<i64> {
        let res = self.eval(&self.ast)?;
        Ok(res)
    }

    fn eval(&self, ast: &Ast) -> Result<i64> {
        let res = match ast {
            Ast::Int(i) => *i,
            Ast::BinOp { op, l, r } => match op {
                BinOp::Add => {
                    let l = self.eval(l)?;
                    let r = self.eval(r)?;
                    l + r
                }
                BinOp::Sub => {
                    let l = self.eval(l)?;
                    let r = self.eval(r)?;
                    l - r
                }
                BinOp::Mul => {
                    let l = self.eval(l)?;
                    let r = self.eval(r)?;
                    l * r
                }
                BinOp::Div => {
                    let l = self.eval(l)?;
                    let r = self.eval(r)?;
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
        let actual = Interpreter::new(code).unwrap().run().unwrap();
        let expect = 3;
        assert_eq!(expect, actual);
    }
}

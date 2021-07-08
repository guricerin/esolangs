use std::{
    collections::HashMap,
    io::{self, BufWriter, Write},
};

use anyhow::{Context, Result};

use crate::{ast::*, parser, token};

#[derive(Debug)]
pub struct Interpreter {
    // Bolicの変数はすべてグローバル変数
    sym_table: HashMap<char, i64>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            sym_table: HashMap::new(),
        }
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

    fn e_stmt(&mut self, stmt: &Stmt) -> Result<()> {
        match stmt {
            Stmt::Expr(Expr::Var(Variable::Assign { var, expr })) => {
                let value = self.e_expr(expr)?;
                // 名前が重複する変数の場合は上書き
                self.sym_table.insert(*var, value);
            }
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
            _ => {
                let msg = format!("interpreter error: unexpected statement: {:?}", stmt);
                return Err(anyhow::anyhow!(msg));
            }
        };
        Ok(())
    }

    fn e_expr(&mut self, ast: &Expr) -> Result<i64> {
        match ast {
            Expr::Var(Variable::Int(i)) => Ok(*i),
            Expr::Var(Variable::Var(var)) => self
                .sym_table
                .get(var)
                .with_context(|| {
                    let msg = format!("interpreter error: <{}> is undelared variable.", var);
                    anyhow::anyhow!(msg)
                })
                .and_then(|value| Ok(*value)),
            Expr::Var(Variable::Assign { var, expr }) => {
                let value = self.e_expr(expr)?;
                self.sym_table.insert(*var, value);
                Ok(value)
            }
            Expr::BinOp { op, l, r } => {
                let l = self.e_expr(l)?;
                let r = self.e_expr(r)?;
                match op {
                    BinOp::Add => Ok(l + r),
                    BinOp::Sub => Ok(l - r),
                    BinOp::Mul => Ok(l * r),
                    BinOp::Div => Ok(l / r),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Interpreter;

    #[test]
    fn assgin() {
        let code = "✩ ☜ ①＋②";
        let mut interpreter = Interpreter::new();
        interpreter.run(code).unwrap();
        let actual = interpreter.sym_table.get(&'✩').unwrap();
        let expect = 3;
        assert_eq!(expect, *actual);
    }

    #[test]
    fn assgin2() {
        let code = "✪ ☜ ✩ ☜ ① ＋ ②";
        let mut interpreter = Interpreter::new();
        interpreter.run(code).unwrap();
        let actual = interpreter.sym_table.get(&'✪').unwrap();
        let expect = 3;
        assert_eq!(expect, *actual);
    }
}

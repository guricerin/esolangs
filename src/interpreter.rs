use std::{
    collections::HashMap,
    io::{self, BufWriter, Write},
};

use anyhow::{Context, Result};

use crate::{ast::*, parser, token};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RetVal {
    Int(i64),
    Void,
}

impl RetVal {
    pub fn to_i(&self) -> Result<i64> {
        match self {
            Self::Int(i) => Ok(*i),
            Self::Void => Err(anyhow::anyhow!("the retrun value type is void.")),
        }
    }
}

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

    fn dbg_table(&self) {
        println!("table: {:?}", &self.sym_table);
    }

    fn eval(&mut self, ast: &Ast) -> Result<()> {
        self.e_stmts(&ast)?;
        Ok(())
    }

    fn e_stmts(&mut self, ast: &Ast) -> Result<RetVal> {
        // 最後の文のリターン値を全体のリターン値とする
        let mut res = RetVal::Void;
        match ast {
            Ast::Stmts(stmts) => {
                for stmt in stmts.iter() {
                    res = self.e_stmt(stmt)?;
                }
            }
        };
        Ok(res)
    }

    fn e_stmt(&mut self, stmt: &Stmt) -> Result<RetVal> {
        match stmt {
            Stmt::Expr(expr) => {
                let res = self.e_expr(expr)?;
                Ok(res)
            }
            Stmt::While { .. } => {
                let res = self.e_while(stmt)?;
                Ok(res)
            }
            Stmt::NumOut(expr) => {
                let x = self.e_expr(expr)?.to_i()?.to_string();
                let mut writer = BufWriter::new(io::stdout());
                writer.write(x.as_bytes())?;
                writer.flush()?;
                Ok(RetVal::Void)
            }
            Stmt::CharOut(expr) => {
                let x = self.e_expr(expr)?.to_i()?;
                let mut writer = BufWriter::new(io::stdout());
                // ASCIIコードとみなす
                writer.write(&[x as u8])?;
                writer.flush()?;
                Ok(RetVal::Void)
            }
        }
    }

    fn e_expr(&mut self, expr: &Expr) -> Result<RetVal> {
        match expr {
            Expr::Var(Variable::Int(i)) => Ok(RetVal::Int(*i)),
            Expr::Var(Variable::Var(var)) => self
                .sym_table
                .get(var)
                .with_context(|| {
                    let msg = format!("interpreter error: <{}> is undelared variable.", var);
                    anyhow::anyhow!(msg)
                })
                .and_then(|value| Ok(RetVal::Int(*value))),
            Expr::Var(Variable::Assign { var, expr }) => {
                let value = self.e_expr(expr)?;
                // 名前が重複する変数の場合は上書き
                self.sym_table.insert(*var, value.to_i()?);
                Ok(value)
            }
            Expr::BinOp { op, l, r } => {
                let l = self.e_expr(l)?.to_i()?;
                let r = self.e_expr(r)?.to_i()?;
                match op {
                    BinOp::Add => Ok(RetVal::Int(l + r)),
                    BinOp::Sub => Ok(RetVal::Int(l - r)),
                    BinOp::Mul => Ok(RetVal::Int(l * r)),
                    BinOp::Div => Ok(RetVal::Int(l / r)),
                }
            }
            Expr::If { cond, conseq, alt } => {
                // 0: false, other num: true
                let cond = self.e_expr(cond)?.to_i()?;

                match (cond, alt) {
                    (0, Some(alt)) => {
                        let alt = alt.clone();
                        self.e_stmts(&Ast::Stmts(*alt))
                    }
                    (0, None) => Ok(RetVal::Void),
                    (_, _) => {
                        let conseq = conseq.clone();
                        self.e_stmts(&Ast::Stmts(*conseq))
                    }
                }
            }
        }
    }

    fn e_while(&mut self, wblock: &Stmt) -> Result<RetVal> {
        match wblock {
            Stmt::While { cond, body } => {
                loop {
                    // 0: false, other num: true
                    let cond = self.e_expr(cond)?.to_i()?;
                    if cond == 0 {
                        break;
                    }
                    // todo: lifetime
                    self.e_stmts(&Ast::Stmts(body.clone()))?;
                }
                Ok(RetVal::Void)
            }
            _ => {
                let msg = format!(
                    "interpreter error: the stmt is not while. actual: {:?}",
                    wblock
                );
                Err(anyhow::anyhow!(msg))
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

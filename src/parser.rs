use std::iter::Peekable;

use anyhow::Result;

use crate::ast::*;
use crate::token::{self, Token};

fn err_msg(msg: &str) -> String {
    format!("parse error: {}", msg)
}

macro_rules! eof {
    () => {
        anyhow::anyhow!("parse error: unexpected eof.")
    };
}

macro_rules! unexpected_token {
    ($lexpr:expr) => {{
        let msg = format!("parse error: unexpected token\ntoken: {:?}", $lexpr);
        anyhow::anyhow!(msg)
    }};
}

pub fn parse(tokens: Vec<Token>) -> Result<Ast> {
    let mut tokens = tokens.clone().into_iter().peekable();
    // println!("tokens: {:?}", &tokens);
    let ast = p_stmts(&mut tokens)?;
    match tokens.next() {
        None => Ok(ast),
        Some(tok) => {
            let msg = format!("redundant token: {:?}", &tok);
            Err(anyhow::anyhow!(err_msg(&msg)))
        }
    }
}

fn p_stmts<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast>
where
    Tokens: Iterator<Item = Token>,
{
    let mut stmts = vec![];
    loop {
        match p_stmt(tokens) {
            Ok(ast) => {
                stmts.push(ast);
            }
            Err(_) => break,
        }
    }
    Ok(Ast::Stmts(stmts))
}

fn p_stmt<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Stmt>
where
    Tokens: Iterator<Item = Token>,
{
    // ok_or().and_then()だとその中のクロージャにtokensをわたせない
    match tokens.peek() {
        None => return Err(eof!()),
        _ => (),
    };

    let tok = tokens.peek().unwrap().clone();
    match tok {
        Token::Symbol(_) => {
            let expr = p_expr(tokens)?;
            let stmt = Stmt::Expr(expr);
            Ok(stmt)
        }
        Token::NumOut => {
            tokens.next();
            let expr = p_expr(tokens)?;
            let stmt = Stmt::NumOut(expr);
            Ok(stmt)
        }
        Token::CharOut => {
            tokens.next();
            let expr = p_expr(tokens)?;
            let stmt = Stmt::CharOut(expr);
            Ok(stmt)
        }
        _ => Err(unexpected_token!(tok)),
    }
}

fn p_expr<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Expr>
where
    Tokens: Iterator<Item = Token>,
{
    p_additive(tokens)
}

fn p_additive<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Expr>
where
    Tokens: Iterator<Item = Token>,
{
    let left = p_multiply(tokens)?;

    if let Some(tok) = tokens.peek() {
        match tok {
            Token::Plus => {
                tokens.next();
                let right = p_expr(tokens)?;
                let ast = Expr::binop(BinOp::Add, left, right);
                Ok(ast)
            }
            Token::Minus => {
                tokens.next();
                let right = p_expr(tokens)?;
                let ast = Expr::binop(BinOp::Sub, left, right);
                Ok(ast)
            }
            _ => Ok(left),
        }
    } else {
        // ほんまか？
        Ok(left)
    }
}

fn p_multiply<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Expr>
where
    Tokens: Iterator<Item = Token>,
{
    let left = p_variable(tokens)?;

    if let Some(tok) = tokens.peek() {
        match tok {
            Token::Mul => {
                tokens.next();
                let right = p_multiply(tokens)?;
                let ast = Expr::binop(BinOp::Mul, Expr::Var(left), right);
                Ok(ast)
            }
            Token::Div => {
                tokens.next();
                let right = p_multiply(tokens)?;
                let ast = Expr::binop(BinOp::Div, Expr::Var(left), right);
                Ok(ast)
            }
            _ => Ok(Expr::Var(left)),
        }
    } else {
        // ほんまか？
        Ok(Expr::Var(left))
    }
}

fn p_variable<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Variable>
where
    Tokens: Iterator<Item = Token>,
{
    match tokens.peek() {
        None => return Err(eof!()),
        _ => (),
    };

    let tok = tokens.peek().unwrap().clone();
    match tok {
        Token::Symbol(var) => {
            tokens.next();
            match p_assgin(tokens) {
                Ok(_) => {
                    let expr = p_expr(tokens)?;
                    let res = Variable::assign(var, expr);
                    Ok(res)
                }
                Err(_) => {
                    let res = Variable::Var(var);
                    Ok(res)
                }
            }
        }
        Token::Num(_) => p_number(tokens),
        _ => Err(unexpected_token!(tok)),
    }
}

fn p_assgin<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<()>
where
    Tokens: Iterator<Item = Token>,
{
    // match tokens.peek() {
    //     None => return Err(eof!()),
    //     _ => (),
    // };

    // let tok = tokens.peek().unwrap();
    // match tok {
    //     Token::Assign => {
    //         tokens.next();
    //         Ok(())
    //     }
    //     _ => Err(unexpected_token!(tok)),
    // }
    let _ = tokens.peek().ok_or(eof!()).and_then(|tok| match tok {
        Token::Assign => Ok(()),
        _ => Err(unexpected_token!(tok)),
    })?;
    tokens.next();
    Ok(())
}

fn p_number<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Variable>
where
    Tokens: Iterator<Item = Token>,
{
    let int = tokens.peek().ok_or(eof!()).and_then(|tok| match tok {
        Token::Num(n) => Ok(Variable::Int(*n as i64)),
        _ => {
            let msg = format!("the token is not number\ntoken: {:?}", tok);
            Err(anyhow::anyhow!(err_msg(&msg)))
        }
    })?;

    tokens.next();
    Ok(int)
}

#[cfg(test)]
mod tests {
    use crate::ast::*;
    use crate::parser;
    use crate::token;

    #[test]
    fn numbers() {
        let numbers = (*token::NUMBERS).clone();
        for (i, n) in numbers.chars().enumerate() {
            let tokens = token::lex(&n.to_string()).unwrap();
            let mut tokens = tokens.into_iter().peekable();
            let expr = parser::p_expr(&mut tokens).unwrap();
            let expect = Expr::int(i as i64);
            assert_eq!(expect, expr);
        }
    }

    #[test]
    fn mul() {
        let code = "①×②";
        let tokens = token::lex(code).unwrap();
        eprintln!("{:?}", &tokens);
        let mut tokens = tokens.into_iter().peekable();
        let expr = parser::p_expr(&mut tokens).unwrap();
        let expect = Expr::binop(BinOp::Mul, Expr::int(1), Expr::int(2));
        assert_eq!(expect, expr);
    }

    #[test]
    fn add() {
        let code = "①＋②";
        let tokens = token::lex(code).unwrap();
        eprintln!("{:?}", &tokens);
        let mut tokens = tokens.into_iter().peekable();
        let expr = parser::p_expr(&mut tokens).unwrap();
        let expect = Expr::binop(BinOp::Add, Expr::int(1), Expr::int(2));
        assert_eq!(expect, expr);
    }

    #[test]
    fn add_mul() {
        let code = "①×②＋③×④";
        let tokens = token::lex(code).unwrap();
        eprintln!("{:?}", &tokens);
        let mut tokens = tokens.into_iter().peekable();
        let expr = parser::p_expr(&mut tokens).unwrap();

        let left = Expr::binop(BinOp::Mul, Expr::int(1), Expr::int(2));
        let right = Expr::binop(BinOp::Mul, Expr::int(3), Expr::int(4));
        let expect = Expr::binop(BinOp::Add, left, right);
        eprintln!("{:?}", &expect);

        assert_eq!(expect, expr);
    }

    #[test]
    fn mul3() {
        let code = "①×②×③×④";
        let tokens = token::lex(code).unwrap();
        eprintln!("{:?}", &tokens);
        let mut tokens = tokens.into_iter().peekable();
        let expr = parser::p_expr(&mut tokens).unwrap();

        let expect = Expr::binop(BinOp::Mul, Expr::int(1), Expr::int(2));
        let expect = Expr::binop(BinOp::Mul, expect, Expr::int(3));
        let expect = Expr::binop(BinOp::Mul, expect, Expr::int(4));
        eprintln!("{:?}", &expect);

        assert_eq!(expect, expr);
    }

    #[test]
    fn numout() {
        let code = "✍①×②＋③×④";
        let tokens = token::lex(code).unwrap();
        eprintln!("{:?}", &tokens);
        let ast = parser::parse(tokens).unwrap();

        let left = Expr::binop(BinOp::Mul, Expr::int(1), Expr::int(2));
        let right = Expr::binop(BinOp::Mul, Expr::int(3), Expr::int(4));
        let expr = Expr::binop(BinOp::Add, left, right);
        let expect = Ast::Stmts(vec![Stmt::NumOut(expr)]);
        eprintln!("{:?}", &expect);

        assert_eq!(expect, ast);
    }

    #[test]
    fn assgin() {
        let code = "✩ ☜ ④";
        let tokens = token::lex(code).unwrap();
        let ast = parser::parse(tokens).unwrap();

        let expect = Variable::assign('✩', Expr::int(4));
        let expect = Ast::Stmts(vec![Stmt::Expr(Expr::Var(expect))]);
        assert_eq!(expect, ast);
    }
}

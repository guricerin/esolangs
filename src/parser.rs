use std::iter::Peekable;

use anyhow::Result;

use crate::ast::*;
use crate::token::{self, Token};

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    cursor: usize,
}

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
        Err(anyhow::anyhow!(msg))
    }};
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens,
            cursor: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Ast> {
        let mut tokens = self.tokens.clone().into_iter().peekable();
        // println!("tokens: {:?}", &tokens);
        let ast = self.p_output(&mut tokens)?;
        match tokens.next() {
            None => Ok(ast),
            Some(tok) => {
                let msg = format!("redundant token: {:?}", &tok);
                Err(anyhow::anyhow!(err_msg(&msg)))
            }
        }
    }

    fn p_output<Tokens>(&self, tokens: &mut Peekable<Tokens>) -> Result<Ast>
    where
        Tokens: Iterator<Item = Token>,
    {
        // ok_or().and_then()だとその中のクロージャでtokensを他のメソッドにわたせない
        match tokens.peek() {
            None => return Err(eof!()),
            _ => (),
        };

        let tok = tokens.peek().unwrap().clone();
        match tok {
            Token::NumOut => {
                tokens.next();
                let expr = self.p_expr(tokens)?;
                let ast = Ast::stmt(Stmt::NumOut, expr);
                Ok(ast)
            }
            Token::CharOut => {
                tokens.next();
                let expr = self.p_expr(tokens)?;
                let ast = Ast::stmt(Stmt::CharOut, expr);
                Ok(ast)
            }
            _ => {
                let expr = self.p_expr(tokens)?;
                let ast = Ast::Expr(expr);
                Ok(ast)
            }
        }
    }

    fn p_expr<Tokens>(&self, tokens: &mut Peekable<Tokens>) -> Result<Expr>
    where
        Tokens: Iterator<Item = Token>,
    {
        self.p_additive(tokens)
    }

    fn p_additive<Tokens>(&self, tokens: &mut Peekable<Tokens>) -> Result<Expr>
    where
        Tokens: Iterator<Item = Token>,
    {
        let left = self.p_multiply(tokens)?;

        if let Some(tok) = tokens.peek() {
            match tok {
                Token::Plus => {
                    tokens.next();
                    let right = self.p_expr(tokens)?;
                    let ast = Expr::binop(BinOp::Add, left, right);
                    Ok(ast)
                }
                Token::Minus => {
                    tokens.next();
                    let right = self.p_expr(tokens)?;
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

    fn p_multiply<Tokens>(&self, tokens: &mut Peekable<Tokens>) -> Result<Expr>
    where
        Tokens: Iterator<Item = Token>,
    {
        let left = self.p_number(tokens)?;

        if let Some(tok) = tokens.peek() {
            match tok {
                Token::Mul => {
                    tokens.next();
                    let right = self.p_multiply(tokens)?;
                    let ast = Expr::binop(BinOp::Mul, left, right);
                    Ok(ast)
                }
                Token::Div => {
                    tokens.next();
                    let right = self.p_multiply(tokens)?;
                    let ast = Expr::binop(BinOp::Div, left, right);
                    Ok(ast)
                }
                _ => Ok(left),
            }
        } else {
            // ほんまか？
            Ok(left)
        }
    }

    fn p_number<Tokens>(&self, tokens: &mut Peekable<Tokens>) -> Result<Expr>
    where
        Tokens: Iterator<Item = Token>,
    {
        let int = tokens.peek().ok_or(eof!()).and_then(|tok| match tok {
            Token::Num(n) => Ok(Expr::Int(*n as i64)),
            _ => {
                let msg = format!("the token is not number\ntoken: {:?}", tok);
                Err(anyhow::anyhow!(err_msg(&msg)))
            }
        })?;

        tokens.next();
        Ok(int)
    }
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
            let ast = parser::Parser::new(tokens).parse().unwrap();
            let expect = Ast::Expr(Expr::Int(i as i64));
            assert_eq!(expect, ast);
        }
    }

    #[test]
    fn mul() {
        let code = "①×②";
        let tokens = token::lex(code).unwrap();
        eprintln!("{:?}", &tokens);
        let ast = parser::Parser::new(tokens).parse().unwrap();
        let expect = Expr::binop(BinOp::Mul, Expr::Int(1), Expr::Int(2));
        let expect = Ast::Expr(expect);
        assert_eq!(expect, ast);
    }

    #[test]
    fn add() {
        let code = "①＋②";
        let tokens = token::lex(code).unwrap();
        eprintln!("{:?}", &tokens);
        let ast = parser::Parser::new(tokens).parse();
        eprintln!("{:?}", &ast);
        let ast = ast.unwrap();
        let expect = Expr::binop(BinOp::Add, Expr::Int(1), Expr::Int(2));
        let expect = Ast::Expr(expect);
        assert_eq!(expect, ast);
    }

    #[test]
    fn add_mul() {
        let code = "①×②＋③×④";
        let tokens = token::lex(code).unwrap();
        eprintln!("{:?}", &tokens);
        let ast = parser::Parser::new(tokens).parse().unwrap();

        let left = Expr::binop(BinOp::Mul, Expr::Int(1), Expr::Int(2));
        let right = Expr::binop(BinOp::Mul, Expr::Int(3), Expr::Int(4));
        let expect = Expr::binop(BinOp::Add, left, right);
        let expect = Ast::Expr(expect);
        eprintln!("{:?}", &expect);

        assert_eq!(expect, ast);
    }

    #[test]
    fn mul3() {
        let code = "①×②×③×④";
        let tokens = token::lex(code).unwrap();
        eprintln!("{:?}", &tokens);
        let ast = parser::Parser::new(tokens).parse().unwrap();

        let expr = Expr::binop(BinOp::Mul, Expr::Int(1), Expr::Int(2));
        let expr = Expr::binop(BinOp::Mul, expr, Expr::Int(3));
        let expect = Expr::binop(BinOp::Mul, expr, Expr::Int(4));
        let expect = Ast::Expr(expect);
        eprintln!("{:?}", &expect);

        assert_eq!(expect, ast);
    }

    #[test]
    fn numout() {
        let code = "✍①×②＋③×④";
        let tokens = token::lex(code).unwrap();
        eprintln!("{:?}", &tokens);
        let ast = parser::Parser::new(tokens).parse().unwrap();

        let left = Expr::binop(BinOp::Mul, Expr::Int(1), Expr::Int(2));
        let right = Expr::binop(BinOp::Mul, Expr::Int(3), Expr::Int(4));
        let expr = Expr::binop(BinOp::Add, left, right);
        let expect = Ast::stmt(Stmt::NumOut, expr);
        eprintln!("{:?}", &expect);

        assert_eq!(expect, ast);
    }
}

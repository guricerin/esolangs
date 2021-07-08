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
    ($tok:expr) => {{
        let msg = format!("parse error: unexpected token\ntoken: {:?}", $tok);
        anyhow::anyhow!(msg)
    }};
}

pub fn parse(tokens: Vec<Token>) -> Result<Ast> {
    let mut tokens = tokens.clone().into_iter().peekable();
    let stmts = p_stmts(&mut tokens, vec![])?;
    match tokens.next() {
        None => Ok(Ast::Stmts(stmts)),
        Some(tok) => {
            let msg = format!("redundant token: {:?}", &tok);
            Err(anyhow::anyhow!(err_msg(&msg)))
        }
    }
}

fn p_stmts<Tokens>(tokens: &mut Peekable<Tokens>, terminators: Vec<Token>) -> Result<Stmts>
where
    Tokens: Iterator<Item = Token>,
{
    let mut stmts = vec![];

    while tokens.peek().is_some() {
        let tok = tokens.peek().unwrap().clone();
        if terminators.contains(&tok) {
            break;
        } else {
            match p_stmt(tokens) {
                Ok(stmt) => {
                    stmts.push(stmt);
                }
                Err(_) => {
                    break;
                }
            }
        }
    }

    Ok(stmts)
}

fn p_stmt<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Stmt>
where
    Tokens: Iterator<Item = Token>,
{
    // ok_or().and_then()だとその中のクロージャにtokensをわたせない
    if tokens.peek().is_none() {
        return Err(eof!());
    }

    let tok = tokens.peek().unwrap().clone();
    match tok {
        Token::Symbol(_) => {
            let expr = p_expr(tokens)?;
            let stmt = Stmt::Expr(expr);
            Ok(stmt)
        }
        Token::While => {
            let stmt = p_while(tokens)?;
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
        _ => {
            let expr = p_expr(tokens)?;
            let stmt = Stmt::Expr(expr);
            Ok(stmt)
        }
    }
}

fn p_while<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Stmt>
where
    Tokens: Iterator<Item = Token>,
{
    consume(tokens, &Token::While)?;
    let cond = p_expr(tokens)?;
    consume(tokens, &Token::Do)?;
    let body = p_stmts(tokens, vec![Token::WhileEnd])?;
    tokens.next();

    let stmt = Stmt::While { cond, body };
    Ok(stmt)
}

fn p_expr<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Expr>
where
    Tokens: Iterator<Item = Token>,
{
    if tokens.peek().is_none() {
        return Err(eof!());
    }

    let tok = tokens.peek().unwrap().clone();
    match tok {
        Token::If => p_if(tokens),
        _ => p_additive(tokens),
    }
}

fn p_additive<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Expr>
where
    Tokens: Iterator<Item = Token>,
{
    let left = p_multiply(tokens)?;
    if tokens.peek().is_none() {
        return Ok(left);
    }

    let tok = tokens.peek().unwrap().clone();
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
}

fn p_multiply<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Expr>
where
    Tokens: Iterator<Item = Token>,
{
    let left = p_variable(tokens)?;
    if tokens.peek().is_none() {
        return Ok(Expr::Var(left));
    }

    let tok = tokens.peek().unwrap().clone();
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
}

fn p_variable<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Variable>
where
    Tokens: Iterator<Item = Token>,
{
    if tokens.peek().is_none() {
        return Err(eof!());
    }

    let tok = tokens.peek().unwrap().clone();
    match tok {
        Token::Symbol(var) => {
            tokens.next();
            match consume(tokens, &Token::Assign) {
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

fn p_if<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Expr>
where
    Tokens: Iterator<Item = Token>,
{
    consume(tokens, &Token::If)?;
    let cond = p_expr(tokens)?;
    consume(tokens, &Token::Then)?;
    let conseq = p_stmts(tokens, vec![Token::Else, Token::IfEnd])?;

    if consume(tokens, &Token::Else).is_ok() {
        let alt = p_stmts(tokens, vec![Token::IfEnd])?;
        tokens.next();
        let res = Expr::if_alt(cond, conseq, alt);
        Ok(res)
    } else if consume(tokens, &Token::IfEnd).is_ok() {
        let res = Expr::if_without_alt(cond, conseq);
        Ok(res)
    } else {
        let msg = format!("the if block has not end token.");
        Err(anyhow::anyhow!(msg))
    }
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

fn consume<Tokens>(tokens: &mut Peekable<Tokens>, expect: &Token) -> Result<()>
where
    Tokens: Iterator<Item = Token>,
{
    tokens.peek().ok_or(eof!()).and_then(|tok| {
        if tok == expect {
            Ok(())
        } else {
            Err(unexpected_token!(tok))
        }
    })?;

    tokens.next();
    Ok(())
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
    fn sub() {
        let code = "①−②";
        let tokens = token::lex(code).unwrap();
        eprintln!("{:?}", &tokens);
        let mut tokens = tokens.into_iter().peekable();
        let expr = parser::p_expr(&mut tokens).unwrap();
        let expect = Expr::binop(BinOp::Sub, Expr::int(1), Expr::int(2));
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

    #[test]
    fn if_expr() {
        let code = "✈①☺②☹③☻";
        let tokens = token::lex(code).unwrap();
        let ast = parser::parse(tokens).unwrap();

        let cond = Expr::int(1);
        let conseq = Expr::int(2);
        let alt = Expr::int(3);
        let expect = Ast::Stmts(vec![Stmt::Expr(Expr::if_alt(
            cond,
            vec![Stmt::Expr(conseq)],
            vec![Stmt::Expr(alt)],
        ))]);
        assert_eq!(expect, ast);
    }

    #[test]
    fn if_expr2() {
        let code = "✈①☺②☻";
        let tokens = token::lex(code).unwrap();
        let ast = parser::parse(tokens).unwrap();

        let cond = Expr::int(1);
        let conseq = Expr::int(2);
        let expect = Ast::Stmts(vec![Stmt::Expr(Expr::if_without_alt(
            cond,
            vec![Stmt::Expr(conseq)],
        ))]);
        assert_eq!(expect, ast);
    }

    #[test]
    fn update_sub() {
        let code = "✪☜ ✪−①";
        let tokens = token::lex(code).unwrap();
        let ast = parser::parse(tokens).unwrap();

        let expect = Variable::assign(
            '✪',
            Expr::binop(BinOp::Sub, Expr::Var(Variable::Var('✪')), Expr::int(1)),
        );
        let expect = Ast::Stmts(vec![Stmt::Expr(Expr::Var(expect))]);
        assert_eq!(expect, ast);
    }

    #[test]
    fn update_add() {
        let code = "✪☜ ✪＋①";
        let tokens = token::lex(code).unwrap();
        let ast = parser::parse(tokens).unwrap();

        let expect = Variable::assign(
            '✪',
            Expr::binop(BinOp::Add, Expr::Var(Variable::Var('✪')), Expr::int(1)),
        );
        let expect = Ast::Stmts(vec![Stmt::Expr(Expr::Var(expect))]);
        assert_eq!(expect, ast);
    }

    #[test]
    fn update_mul() {
        let code = "✪☜ ✪×①";
        let tokens = token::lex(code).unwrap();
        let ast = parser::parse(tokens).unwrap();

        let expect = Variable::assign(
            '✪',
            Expr::binop(BinOp::Mul, Expr::Var(Variable::Var('✪')), Expr::int(1)),
        );
        let expect = Ast::Stmts(vec![Stmt::Expr(Expr::Var(expect))]);
        assert_eq!(expect, ast);
    }

    #[test]
    fn update_div() {
        let code = "✪☜ ✪÷①";
        let tokens = token::lex(code).unwrap();
        let ast = parser::parse(tokens).unwrap();

        let expect = Variable::assign(
            '✪',
            Expr::binop(BinOp::Div, Expr::Var(Variable::Var('✪')), Expr::int(1)),
        );
        let expect = Ast::Stmts(vec![Stmt::Expr(Expr::Var(expect))]);
        assert_eq!(expect, ast);
    }
}

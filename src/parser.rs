use std::fmt::format;

use anyhow::Result;

use crate::ast::{self, Ast};
use crate::token::{self, Token};

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    cursor: usize,
}

fn err_msg(msg: &str) -> String {
    format!("parse error: {}", msg)
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens,
            cursor: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Ast> {
        self.p_number()
    }

    fn p_number(&mut self) -> Result<Ast> {
        let ast = match self.tokens[self.cursor].clone() {
            Token::Num(n) => Ast::Int(n as i64),
        };
        self.cursor += 1;

        Ok(ast)
    }
}

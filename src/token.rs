use std::str::Chars;

use anyhow::Result;
use once_cell::sync::Lazy;

#[derive(Debug, Clone)]
pub enum Token {
    Num(u8),
}

static NUMBERS: Lazy<String> = Lazy::new(|| format!("⓪①②③④⑤⑥⑦⑧⑨⑩"));

pub fn lex(code: String) -> Result<Vec<Token>> {
    let mut tokens = vec![];
    for ch in code.chars() {
        match ch {
            // 数字のlexについてはchar_indices()とか使えそうで使えなかったので、愚直にやることにした
            '⓪' => {
                tokens.push(Token::Num(0));
            }
            '①' => {
                tokens.push(Token::Num(1));
            }
            '②' => {
                tokens.push(Token::Num(2));
            }
            '③' => {
                tokens.push(Token::Num(3));
            }
            '④' => {
                tokens.push(Token::Num(4));
            }
            '⑤' => {
                tokens.push(Token::Num(5));
            }
            '⑥' => {
                tokens.push(Token::Num(6));
            }
            '⑦' => {
                tokens.push(Token::Num(7));
            }
            '⑧' => {
                tokens.push(Token::Num(8));
            }
            '⑨' => {
                tokens.push(Token::Num(9));
            }
            '⑩' => {
                tokens.push(Token::Num(10));
            }
            _ => (),
        }
    }
    Ok(tokens)
}

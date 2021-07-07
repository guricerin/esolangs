use anyhow::Result;
use once_cell::sync::Lazy;

#[derive(Debug, Clone)]
pub enum Token {
    Num(u8),
    Plus,
    Minus,
    Mul,
    Div,
}

pub static NUMBERS: Lazy<String> = Lazy::new(|| format!("⓪①②③④⑤⑥⑦⑧⑨⑩"));

pub fn lex(code: &str) -> Result<Vec<Token>> {
    let mut tokens = vec![];
    for ch in code.chars() {
        match ch {
            '⓪' | '①' | '②' | '③' | '④' | '⑤' | '⑥' | '⑦' | '⑧' | '⑨' | '⑩' => {
                for (i, x) in (*NUMBERS).chars().enumerate() {
                    if ch == x {
                        tokens.push(Token::Num(i as u8));
                        break;
                    }
                }
            }
            '＋' => {
                tokens.push(Token::Plus);
            }
            '－' => {
                tokens.push(Token::Minus);
            }
            '×' => {
                tokens.push(Token::Mul);
            }
            '÷' => {
                tokens.push(Token::Div);
            }
            _ => (),
        }
    }
    Ok(tokens)
}

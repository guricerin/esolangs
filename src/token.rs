use anyhow::Result;
use once_cell::sync::Lazy;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Num(u8),
    Plus,
    Minus,
    Mul,
    Div,
    NumOut,
    CharOut,
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
            // 鉛筆
            '\u{270d}' => {
                tokens.push(Token::NumOut);
            }
            // 音符
            '\u{266a}' => {
                tokens.push(Token::CharOut);
            }
            _ => (),
        }
    }
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbers() {
        let numbers = (*NUMBERS).clone();
        let actual = lex(&numbers).unwrap();
        let mut expect = vec![];
        for i in 0..=10 {
            expect.push(Token::Num(i));
        }
        assert_eq!(expect, actual);
    }

    #[test]
    fn op() {
        let code = "＋－×÷";
        let actual = lex(code).unwrap();
        let expect = vec![Token::Plus, Token::Minus, Token::Mul, Token::Div];
        assert_eq!(expect, actual);
    }

    #[test]
    fn output() {
        let code = "✍♪";
        let actual = lex(code).unwrap();
        let expect = vec![Token::NumOut, Token::CharOut];
        assert_eq!(expect, actual);
    }
}

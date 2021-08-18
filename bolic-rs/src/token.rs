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
    Symbol(char),
    Assign,
    If,
    Then,
    Else,
    IfEnd,
    While,
    Do,
    WhileEnd,
}

// 10はLFのASCIIコード
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
            '−' => {
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
            '✪' | '✷' | '✲' | '✩' => {
                tokens.push(Token::Symbol(ch));
            }
            '☜' => {
                tokens.push(Token::Assign);
            }
            '✈' => {
                tokens.push(Token::If);
            }
            '☺' => {
                tokens.push(Token::Then);
            }
            '☹' => {
                tokens.push(Token::Else);
            }
            '☻' => {
                tokens.push(Token::IfEnd);
            }
            '♺' => {
                tokens.push(Token::While);
            }
            '☞' => {
                tokens.push(Token::Do);
            }
            '♘' => {
                tokens.push(Token::WhileEnd);
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
        let code = "＋−×÷";
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

    #[test]
    fn var() {
        let code = "✪  ✷  ✲ | ✩";
        let actual = lex(code).unwrap();
        let expect = vec![
            Token::Symbol('✪'),
            Token::Symbol('✷'),
            Token::Symbol('✲'),
            Token::Symbol('✩'),
        ];
        assert_eq!(expect, actual);
    }

    #[test]
    fn assign() {
        let code = "✩ ☜ ④";
        let actual = lex(code).unwrap();
        let expect = vec![Token::Symbol('✩'), Token::Assign, Token::Num(4)];
        assert_eq!(expect, actual);
    }

    #[test]
    fn assign2() {
        let code = "✩ ☜ ✲";
        let actual = lex(code).unwrap();
        let expect = vec![Token::Symbol('✩'), Token::Assign, Token::Symbol('✲')];
        assert_eq!(expect, actual);
    }

    #[test]
    fn assign3() {
        let code = "✪☜ ✪−①";
        let actual = lex(code).unwrap();
        let expect = vec![
            Token::Symbol('✪'),
            Token::Assign,
            Token::Symbol('✪'),
            Token::Minus,
            Token::Num(1),
        ];
        assert_eq!(expect, actual);
    }

    #[test]
    fn if_then_else_end() {
        let code = "✈☺☹☻";
        let actual = lex(code).unwrap();
        let expect = vec![Token::If, Token::Then, Token::Else, Token::IfEnd];
        assert_eq!(expect, actual);
    }
}

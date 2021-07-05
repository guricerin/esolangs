use anyhow::Result;
use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Space,
    Tab,
    Lf,
}

pub fn tokenize(code: &str) -> Result<Vec<Token>> {
    let code = bleach(code)?;
    let code = code.as_bytes();
    let mut tokens = vec![];
    for c in code.iter() {
        match c {
            b' ' => tokens.push(Token::Space),
            b'\t' => tokens.push(Token::Tab),
            b'\n' => tokens.push(Token::Lf),
            _ => (),
        }
    }
    Ok(tokens)
}

/// 空白以外の文字をすべて除去
fn bleach(s: &str) -> Result<String> {
    let regex = Regex::new(r"[^\x20\t\n]")?;
    let res = regex.replace_all(s, "");
    Ok(res.to_string())
}

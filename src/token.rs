use anyhow::Result;
use regex::Regex;

#[derive(Debug, Clone)]
pub enum Token {
    Plus,
    Star,
    Space,
    Period,
    Comma,
    Quote,
    BQuote,
}

pub fn tokenize(code: &str) -> Result<Vec<Token>> {
    let code = bleach(code)?;
    let code = code.as_bytes();
    let mut tokens = vec![];
    for c in code.iter() {
        let tok = match c {
            b' ' => Token::Space,
            b'+' => Token::Plus,
            b'*' => Token::Star,
            b'.' => Token::Period,
            b',' => Token::Comma,
            b'\'' => Token::Quote,
            b'`' => Token::BQuote,
            _ => return Err(anyhow::anyhow!("imcomplete bleaching.")),
        };
        tokens.push(tok);
    }
    Ok(tokens)
}

fn bleach(s: &str) -> Result<String> {
    let regex = Regex::new(r"[^\+\*\.\,\`\'\x20]")?;
    let res = regex.replace_all(s, "");
    Ok(res.to_string())
}

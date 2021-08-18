#[derive(Debug)]
pub enum Token {
    Hello,
    Quine,
    Ninety,
    Plus,
    Ignore,
}

pub fn tokenize(s: &str) -> Vec<Token> {
    let tokens: Vec<_> = s
        .chars()
        .map(|c| match c {
            'H' => Token::Hello,
            'Q' => Token::Quine,
            '9' => Token::Ninety,
            '+' => Token::Plus,
            _ => Token::Ignore,
        })
        .collect();
    tokens
}

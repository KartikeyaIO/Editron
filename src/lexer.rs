use std::fs;

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
    pub line: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Identifier,
    Num,
    String,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Plus,
    Minus,
    Star,
    Slash,
    Equal,
    GreaterThan,
    LessThan,
    If,
    Fn,
    Let,
    EOF,
}
fn TokenToChr(c: char) -> Option<TokenKind> {
    match c {
        '(' => Some(TokenKind::LeftParen),
        ')' => Some(TokenKind::RightParen),
        '{' => Some(TokenKind::LeftBrace),
        '}' => Some(TokenKind::RightBrace),
        '+' => Some(TokenKind::Plus),
        '-' => Some(TokenKind::Minus),
        '*' => Some(TokenKind::Star),
        '/' => Some(TokenKind::Slash),
        '=' => Some(TokenKind::Equal),
        '>' => Some(TokenKind::GreaterThan),
        '<' => Some(TokenKind::LessThan),
        _ => None,
    }
}

fn main() {
    let source = fs::read_to_string("input.edt");
    let mut data: Vec<Token> = Vec::new();
    let mut line;

    for i in source.chars() {
        if i == '\n' {
            line += 1;
            continue;
        }
        if i == ' ' && i == '\t' {
            continue;
        }
        if let Some(kind) = TokenToChr(i) {}
    }
}

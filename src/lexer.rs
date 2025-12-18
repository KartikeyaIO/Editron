use std::fs;
use std::io;

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
    Else,
    Fn,
    Let,
    And,
    Or,
    Not,
    Return,
    EOF,
}

#[derive(Debug, Clone)]
pub enum State {
    Default,
    Identifier,
    String,
}

pub fn char_to_token(c: char) -> Option<TokenKind> {
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

fn identify_token(s: &str) -> Option<TokenKind> {
    match s {
        "if" => Some(TokenKind::If),
        "else" => Some(TokenKind::Else),
        "and" => Some(TokenKind::And),
        "or" => Some(TokenKind::Or),
        "not" => Some(TokenKind::Not),
        "return" => Some(TokenKind::Return),
        "fn" => Some(TokenKind::Fn),
        "let" => Some(TokenKind::Let),
        _ => Some(TokenKind::Identifier),
    }
}

pub fn lexer() -> Result<Vec<Token>, io::Error> {
    let source = fs::read_to_string("input.edt")?;
    let bytes = source.as_bytes();
    let len = bytes.len();

    let mut tokens = Vec::new();
    let mut line = 1;
    let mut i = 0;

    let mut state = State::Default;
    let mut buffer = String::new();

    while i < len {
        let c = bytes[i] as char;

        match state {
            State::Default => match c {
                '\n' => {
                    line += 1;
                    i += 1;
                }
                ' ' | '\t' => {
                    i += 1;
                }
                '"' => {
                    buffer.clear();
                    state = State::String;
                    i += 1;
                }
                c if c.is_ascii_alphabetic() || c == '_' => {
                    buffer.clear();
                    buffer.push(c);
                    state = State::Identifier;
                    i += 1;
                }
                c => {
                    if let Some(kind) = char_to_token(c) {
                        tokens.push(Token {
                            kind,
                            value: c.to_string(),
                            line,
                        });
                    }
                    i += 1;
                }
            },

            State::Identifier => {
                if c.is_ascii_alphanumeric() || c == '_' {
                    buffer.push(c);
                    i += 1;
                } else {
                    let kind = identify_token(&buffer).unwrap();
                    tokens.push(Token {
                        kind,
                        value: buffer.clone(),
                        line,
                    });
                    buffer.clear();
                    state = State::Default;
                }
            }

            State::String => {
                if c == '"' {
                    tokens.push(Token {
                        kind: TokenKind::String,
                        value: buffer.clone(),
                        line,
                    });
                    buffer.clear();
                    state = State::Default;
                    i += 1;
                } else {
                    if c == '\n' {
                        line += 1;
                    }
                    buffer.push(c);
                    i += 1;
                }
            }
        }
    }

    tokens.push(Token {
        kind: TokenKind::EOF,
        value: String::new(),
        line,
    });

    Ok(tokens)
}

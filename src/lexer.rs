#[derive(Debug, Clone)]
pub struct Token {
    /// The Token Struct has Three values
    /// kind which decides token kind
    /// value which stores values
    /// line which stores the line numbers
    pub kind: TokenKind,
    pub value: String,
    pub line: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    /// TokenKind enum stores the kind of tokens implimented in Editron
    Identifier,
    Int,
    String,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Plus,
    Minus,
    Star,
    Slash,
    SemiColon,
    Equal,
    GreaterThan,
    LessThan,
    Bool,
    If,
    Else,
    Fn,
    Let,
    And,
    Or,
    Not,
    Return,
    Float,
    EOF,
}

#[derive(Debug, Clone)]
/// State Enum stores the state of the the data as the control moves through each character
pub enum State {
    Default,
    Identifier,
    String,
    Number,
}
#[derive(Debug, Clone)]
pub enum LexError {
    /// LexError generates the the Errors at lexer level
    InvalidCharacter {
        ch: char,
        line: usize,
        message: String,
    }, // The InvalidCharacter Error tells the user the character is invalid

    UnterminatedString {
        line: usize,
        message: String,
    }, // The UnterminatedString Error tells the user the string was not terminated

    InvalidNumber {
        value: String,
        line: usize,
        message: String,
    }, // The Invalid Number Error Tells the user the number is invalid
}

pub fn char_to_token(c: char) -> Option<TokenKind> {
    // converts the character to TokenKind
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
        ';' => Some(TokenKind::SemiColon),
        _ => None,
    }
}

fn identify_token(s: &str) -> Option<TokenKind> {
    // The identify_token function identifies the tokens such as keywords
    match s {
        "if" => Some(TokenKind::If),
        "else" => Some(TokenKind::Else),
        "and" => Some(TokenKind::And),
        "or" => Some(TokenKind::Or),
        "not" => Some(TokenKind::Not),
        "return" => Some(TokenKind::Return),
        "fn" => Some(TokenKind::Fn),
        "let" => Some(TokenKind::Let),
        "True" => Some(TokenKind::Bool),
        "False" => Some(TokenKind::Bool),
        _ => Some(TokenKind::Identifier),
    }
}

pub fn lexer(source: &str) -> Result<Vec<Token>, LexError> {
    // lexer function returns as Vector containing Tokens
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
                ';' => {
                    tokens.push(Token {
                        kind: TokenKind::SemiColon,
                        value: ";".to_string(),
                        line,
                    });
                    i += 1;
                }

                '\n' => {
                    line += 1;
                    i += 1;
                }

                ' ' | '\t' | '\r' => {
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
                c if c.is_ascii_digit() => {
                    buffer.clear();
                    buffer.push(c);
                    state = State::Number;
                    i += 1;
                }
                c => {
                    if let Some(kind) = char_to_token(c) {
                        tokens.push(Token {
                            kind,
                            value: c.to_string(),
                            line,
                        });
                    } else {
                        return Err(LexError::InvalidCharacter {
                            ch: c,
                            line,
                            message: "The Character is invalid".to_string(),
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
            State::Number => {
                if c.is_ascii_digit() {
                    buffer.push(c);
                    i += 1;
                } else if c == '.' && !buffer.contains('.') {
                    buffer.push(c);
                    i += 1;
                } else if c == '.' && buffer.contains('.') {
                    return Err(LexError::InvalidNumber {
                        value: buffer.clone(),
                        line,
                        message: "The Number contains '.' more than once which is not allowed!"
                            .to_string(),
                    });
                } else {
                    let kind = if buffer.contains('.') {
                        TokenKind::Float
                    } else {
                        TokenKind::Int
                    };

                    tokens.push(Token {
                        kind,
                        value: buffer.clone(),
                        line,
                    });

                    buffer.clear();
                    state = State::Default;
                }
            }
        }
    }
    if matches!(state, State::String) {
        return Err(LexError::UnterminatedString {
            line,
            message: "The string was never terminated!".to_string(),
        });
    }
    tokens.push(Token {
        kind: TokenKind::EOF,
        value: String::new(),
        line,
    });

    Ok(tokens)
}

use crate::lexer::{Token, TokenKind};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum OpCode {
    LoadString,
    Add,
    Sub,
    Mul,
    Div,
    Print,
}

#[derive(Debug, Clone)]
pub enum Operand {
    Integer(i64),
    Register(usize),
    StrLiteral(String),
}

#[derive(Debug)]
pub struct Instruction {
    pub op: OpCode,
    pub dest: Option<usize>,
    pub args: Vec<Operand>,
}

#[derive(Debug, Clone)]
enum ParseValue {
    Constant(i64),
    RuntimeReg(usize),
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,

    symbol_table: HashMap<String, ParseValue>,
    ir_program: Vec<Instruction>,

    reg_counter: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            pos: 0,
            symbol_table: HashMap::new(),
            ir_program: Vec::new(),
            reg_counter: 0,
        }
    }

    fn new_register(&mut self) -> usize {
        let r = self.reg_counter;
        self.reg_counter += 1;
        r
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn consume(&mut self) -> &Token {
        let tok = &self.tokens[self.pos];
        if tok.kind != TokenKind::EOF {
            self.pos += 1;
        }
        tok
    }

    fn expect(&mut self, kind: TokenKind) -> &Token {
        let token = self.consume();
        if token.kind != kind {
            panic!(
                "Syntax Error: Expected {:?}, got {:?} at line {}",
                kind, token.kind, token.line
            );
        }
        token
    }

    pub fn parse(&mut self) -> &Vec<Instruction> {
        while self.peek().kind != TokenKind::EOF {
            match self.peek().kind {
                TokenKind::Let => self.parse_let(),
                _ => {
                    self.consume();
                }
            }
        }
        &self.ir_program
    }

    fn parse_let(&mut self) {
        self.consume(); // let
        let name = self.expect(TokenKind::Identifier).value.clone();
        self.expect(TokenKind::Equal);

        let value = self.parse_expression();
        self.symbol_table.insert(name.clone(), value.clone());

        self.expect(TokenKind::SemiColon);
    }

    fn parse_expression(&mut self) -> ParseValue {
        self.parse_term()
    }

    fn parse_term(&mut self) -> ParseValue {
        let mut left = self.parse_factor();

        while matches!(self.peek().kind, TokenKind::Plus | TokenKind::Minus) {
            let op = match self.consume().kind {
                TokenKind::Plus => OpCode::Add,
                TokenKind::Minus => OpCode::Sub,
                _ => unreachable!(),
            };

            let right = self.parse_factor();
            left = self.evaluate_operation(op, left, right);
        }

        left
    }

    fn parse_factor(&mut self) -> ParseValue {
        let mut left = self.parse_primary();

        while matches!(self.peek().kind, TokenKind::Star | TokenKind::Slash) {
            let op = match self.consume().kind {
                TokenKind::Star => OpCode::Mul,
                TokenKind::Slash => OpCode::Div,
                _ => unreachable!(),
            };

            let right = self.parse_primary();
            left = self.evaluate_operation(op, left, right);
        }

        left
    }

    fn parse_primary(&mut self) -> ParseValue {
        let token = self.consume().clone();

        match token.kind {
            TokenKind::Int => ParseValue::Constant(token.value.parse().unwrap()),

            TokenKind::String => {
                let reg = self.new_register();
                self.ir_program.push(Instruction {
                    op: OpCode::LoadString,
                    dest: Some(reg),
                    args: vec![Operand::StrLiteral(token.value)],
                });
                ParseValue::RuntimeReg(reg)
            }

            TokenKind::Identifier => self
                .symbol_table
                .get(&token.value)
                .cloned()
                .unwrap_or_else(|| panic!("Undefined variable: {}", token.value)),

            _ => panic!("Unexpected token in expression: {:?}", token),
        }
    }

    fn evaluate_operation(
        &mut self,
        op: OpCode,
        left: ParseValue,
        right: ParseValue,
    ) -> ParseValue {
        match (&left, &right) {
            (ParseValue::Constant(l), ParseValue::Constant(r)) => {
                let result = match op {
                    OpCode::Add => l + r,
                    OpCode::Sub => l - r,
                    OpCode::Mul => l * r,
                    OpCode::Div => {
                        if *r == 0 {
                            panic!("Division by zero in constant expression");
                        }
                        l / r
                    }
                    _ => unreachable!(),
                };
                ParseValue::Constant(result)
            }

            _ => {
                let dest = self.new_register();

                let to_operand = |v: ParseValue| match v {
                    ParseValue::Constant(c) => Operand::Integer(c),
                    ParseValue::RuntimeReg(r) => Operand::Register(r),
                };

                self.ir_program.push(Instruction {
                    op,
                    dest: Some(dest),
                    args: vec![to_operand(left), to_operand(right)],
                });

                ParseValue::RuntimeReg(dest)
            }
        }
    }
}

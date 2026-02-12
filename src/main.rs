#![allow(warnings, unused)]
use std::fs;
use std::io;

use editron_v1::lexer;
use editron_v1::parser;

fn main() -> Result<(), io::Error> {
    let source = fs::read_to_string("input.edt")?;

    let tokens = match lexer::lexer(&source) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Lexer Error: {:?}", e);
            return Ok(());
        }
    };

    for token in &tokens {
        println!("{:?}", token);
    }

    let mut parser = parser::Parser::new(tokens);
    let ir = parser.parse();

    for instr in ir {
        println!("{:?}", instr);
    }

    Ok(())
}

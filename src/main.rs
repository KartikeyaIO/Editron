#![allow(warnings, unused)]
mod engine;
mod lexer;
mod parser;
use std::fs;
use std::io;

fn main() -> Result<(), io::Error> {
    let source = fs::read_to_string("input.edt")?;
    match lexer::lexer(&source) {
        Ok(tokens) => {
            for token in tokens {
                println!("{:?}", token);
            }
        }
        Err(e) => {
            eprintln!("\nLexer Error: {:?}", e);
        }
    }
    Ok(())
}

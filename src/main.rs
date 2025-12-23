mod lexer;
mod parser;

fn main() {
    match lexer::lexer() {
        Ok(tokens) => {
            for token in tokens {
                println!("{:?}", token);
            }
        }
        Err(e) => {
            eprintln!("Lexer error: {}", e);
        }
    }
}

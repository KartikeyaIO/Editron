use drive::engine::Engine;
use drive::parser;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source = std::fs::read_to_string("input.drive").expect("Failed to read file");

    let program = parser::parse(&source).expect("Parsing Failed!");

    let mut engine = Engine::new();

    engine.run(&program).expect("Execution Error!");

    Ok(())
}

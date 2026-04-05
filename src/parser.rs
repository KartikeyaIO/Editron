use crate::lexer;

pub enum State {
    Load,
    Import,
    Export,
    Assigment,
    FunCall,
}
pub struct Load {
    path: String,
}

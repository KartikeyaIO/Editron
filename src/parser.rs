#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    ExprStmt(Expr),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    String(String),
    Identifier(String),

    Call { callee: Box<Expr>, args: Vec<Expr> },
}
// I have stopped working on parser and started building the core engine because ofcourse the parser is too much without results and I have never worked with trees so I need sometime to study tree structure.
pub enum Command {
    LoadImage { path: String },
    Resize { width: u32, height: u32 },
    Crop { x: u32, y: u32, w: u32, h: u32 },
    Export { path: String },
}

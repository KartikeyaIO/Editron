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
pub enum Command {
    LoadImage { path: String },
    Resize { width: u32, height: u32 },
    Crop { x: u32, y: u32, w: u32, h: u32 },
    Export { path: String },
}

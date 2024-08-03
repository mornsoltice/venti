#[derive(Debug)]
pub enum Expr {
    Number(i64),
    String(String),
    Boolean(bool),
    Identifier(String),
    BInaryOp(Box<Expr>, BinOp, Box<Expr>),
}

#[derive(Debug)]
pub enum BinOp {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug)]
pub enum Statement {
    VariableDeclaration { identifier: String, value: Expr},
    Print(Expr),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Stmt {
    Binding(String, Expr),
    Print(Expr),
    Define(String, Vec<String>, Expr),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expr {
    Value(i32),
    Variable(String),
    Operation(Operator, Box<Expr>, Box<Expr>),
    FuncCall(String, Vec<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Gt,
    Ge,
    Lt,
    Le,
    Eq,
    Neq,
}

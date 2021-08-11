use crate::{MiniError, MiniResult};

pub type Ast = Vec<Stmt>;

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

/// The operator enum for expressions.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Operator {
    /// `+`
    Add,
    /// `-`
    Sub,
    /// `*`
    Mul,
    /// `/`
    Div,
    /// `%`
    Rem,
    /// `>`
    Gt,
    /// `>=`, or `=>`
    Ge,
    /// `<`
    Lt,
    /// `<=`, or `=<`
    Le,
    /// `==`
    Eq,
    /// `!=`
    Neq,
}

pub fn parse<S: AsRef<str>>(input: S) -> MiniResult<Ast> {
    parser::program(input.as_ref()).map_err(MiniError::Parse)
}

peg::parser! { grammar parser() for str {
    rule _ = ("\t"/" "/"\\\n"/"\\\r")*
    rule __ = (_ ("\n"/"\r"))*
    rule space() = ("\t"/" "/"\\\n"/"\\\r")+

    pub rule program() -> Ast
        = stmt()*

    rule stmt() -> Stmt
        = print()
        / binding()
        / define()

    rule print() -> Stmt
        = _ "print" space() e:expr() __ { Stmt::Print(e) }

    rule binding() -> Stmt
        = _ "let" space() v:ident() _ "=" _ e:expr() __ { Stmt::Binding(v, e) }

    rule define() -> Stmt
        = _ "def" space() n:ident() _
          "(" a:((_ a:ident() _ { a }) ** (",")) ","? _ ")" _
          "=" _ e:expr() __ { Stmt::Define(n, a, e) }

    rule expr() -> Expr = eq()

    rule eq() -> Expr
        = l:comp() rs:( _ op:$(("=="/"!=")) _ r:comp() { (op, r) })*
        {
            rs.into_iter().fold(l, |l, (op, r)| Expr::Operation(
                match op {
                    "==" => Operator::Eq,
                    "!=" => Operator::Neq,
                    _ => unreachable!(),
                },
                Box::new(l),
                Box::new(r),
            ))
        }

    rule comp() -> Expr
        = l:add() rs:( _ op:$(("=>"/"=<"/">="/"<="/">"/"<")) _ r:add() { (op, r) })*
        {
            rs.into_iter().fold(l, |l, (op, r)| Expr::Operation(
                match op {
                    ">" => Operator::Gt,
                    "=>" | ">=" => Operator::Ge,
                    "<" => Operator::Lt,
                    "=<" | "<=" => Operator::Le,
                    _ => unreachable!(),
                },
                Box::new(l),
                Box::new(r),
            ))
        }

    rule add() -> Expr
        = l:mul() rs:( _ op:$(("+"/"-")) _ r:mul() { (op, r) })*
        {
            rs.into_iter().fold(l, |l, (op, r)| Expr::Operation(
                match op {
                    "+" => Operator::Add,
                    "-" => Operator::Sub,
                    _ => unreachable!(),
                },
                Box::new(l),
                Box::new(r),
            ))
        }

    rule mul() -> Expr
        = l:atom() rs:( _ op:$(("*"/"/"/"%")) _ r:atom() { (op, r) })*
        {
            rs.into_iter().fold(l, |l, (op, r)| Expr::Operation(
                match op {
                    "*" => Operator::Mul,
                    "/" => Operator::Div,
                    "%" => Operator::Rem,
                    _ => unreachable!(),
                },
                Box::new(l),
                Box::new(r),
            ))
        }

    rule atom() -> Expr
        = n:number() { Expr::Value(n) }
        / "(" _ e:expr() _ ")" { e }
        / funccall()
        / if_expr()
        / v:ident() { Expr::Variable(v) }

    rule funccall() -> Expr
        = n:ident() _ "(" e:((_ e:expr() _ { e }) ** (",")) ","? _ ")" {
            Expr::FuncCall(n, e)
        }

    rule if_expr() -> Expr
        = "if" space() c:expr() space()
          "then" space() t:expr() space()
          "else" space() f:expr() { Expr::If(Box::new(c), Box::new(t), Box::new(f)) }

    rule ident() -> String
        = s:$(['a'..='z' | '_']*) { String::from(s) }

    rule number() -> i32
        = "-" _ n:unsigned() { -n }
        / ("+"/"") _ n:unsigned() { n }

    rule unsigned() -> i32
        = n:$(['0'..='9']+) {? n.parse().or(Err("Integer Parsing Error"))}

}}

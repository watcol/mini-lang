#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expr {
    Value(i32),
    Operation(Operator, Box<Expr>, Box<Expr>),
    FuncCall(String, Vec<Expr>),
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

peg::parser! { grammar parser() for str {
    rule _ = ['\t' | ' ']*
    rule __ = ['\n' | '\r']

    pub rule program() -> Expr
        = e:expr() __ { e }

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

    rule funccall() -> Expr
        = n:ident() _ "(" e:(expr() ** ",") _ ","? _ ")" { Expr::FuncCall(n, e) }

    rule ident() -> String
        = s:$(['a'..='z' | '_']*) { String::from(s) }

    rule number() -> i32
        = "-" _ n:unsigned() { -n }
        / ("+"/"") _ n:unsigned() { n }

    rule unsigned() -> i32
        = n:$(['0'..='9']+) {? n.parse().or(Err("Integer Parsing Error"))}

}}

pub fn parse<S: AsRef<str>>(input: S) -> anyhow::Result<Expr> {
    Ok(parser::program(input.as_ref())?)
}

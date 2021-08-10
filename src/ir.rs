use crate::{parser, MiniResult, MiniError};
use std::collections::HashMap;

pub use parser::Operator;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Program {
    pub funcs: Vec<Expr>,
    pub vars: Vec<Expr>,
    pub prints: Vec<Expr>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expr {
    Value(i32),
    Variable(usize, usize),
    Operation(Operator, Box<Expr>, Box<Expr>),
    FuncCall(usize, Vec<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
}

impl Expr {
    fn from_ast(
        e: parser::Expr,
        ns_vars: &HashMap<String, usize>,
        ns_funcs: &HashMap<String, (usize, usize)>,
    ) -> MiniResult<Self> {
        Ok(match e {
            parser::Expr::Value(v) => Self::Value(v),
            parser::Expr::Variable(s) => {
                Self::Variable(0, *ns_vars.get(&s).ok_or("Using undefined variable.")?)
            }
            parser::Expr::Operation(op, lhs, rhs) => Self::Operation(
                op,
                Box::new(Self::from_ast(*lhs, ns_vars, ns_funcs)?),
                Box::new(Self::from_ast(*rhs, ns_vars, ns_funcs)?),
            ),
            parser::Expr::FuncCall(s, e) => {
                let (id, args) = *ns_funcs.get(&s).ok_or("Using undefined function.")?;
                if e.len() != args {
                    return Err(MiniError::from("Illegal arguments."));
                }
                Self::FuncCall(
                    id,
                    e.into_iter()
                        .map(|e| Self::from_ast(e, ns_vars, ns_funcs))
                        .collect::<Result<Vec<_>, _>>()?,
                )
            }
            parser::Expr::If(c, t, f) => Self::If(
                Box::new(Self::from_ast(*c, ns_vars, ns_funcs)?),
                Box::new(Self::from_ast(*t, ns_vars, ns_funcs)?),
                Box::new(Self::from_ast(*f, ns_vars, ns_funcs)?),
            ),
        })
    }

    pub fn circulate(self, depth: usize) -> Self {
        match self {
            Self::Value(v) => Self::Value(v),
            Self::Variable(_, id) => Self::Variable(depth, id),
            Self::Operation(op, lhs, rhs) => Self::Operation(
                op,
                Box::new((*lhs).circulate(depth)),
                Box::new((*rhs).circulate(depth)),
            ),
            Self::FuncCall(id, args) => {
                Self::FuncCall(id, args.into_iter().map(|e| e.circulate(depth)).collect())
            }
            Self::If(c, t, f) => Self::If(
                Box::new((*c).circulate(depth)),
                Box::new((*t).circulate(depth)),
                Box::new((*f).circulate(depth)),
            ),
        }
    }
}

pub fn compile(ast: parser::Ast) -> MiniResult<Program> {
    let mut vars = Vec::new();
    let mut ns_vars = HashMap::new();
    let mut funcs = Vec::new();
    let mut ns_funcs = HashMap::new();
    let mut prints = Vec::new();
    for stmt in ast {
        match stmt {
            parser::Stmt::Binding(v, e) => {
                let id = vars.len();
                vars.push(Expr::from_ast(e, &ns_vars, &ns_funcs)?);
                ns_vars.insert(v, id);
            }
            parser::Stmt::Print(e) => {
                prints.push(Expr::from_ast(e, &ns_vars, &ns_funcs)?);
            }
            parser::Stmt::Define(f, a, e) => {
                let id = vars.len();
                let args = a.len();
                ns_funcs.insert(f, (id, args));
                let local_vars = a.into_iter().enumerate().map(|(i, s)| (s, i)).collect();
                funcs.push(Expr::from_ast(e, &local_vars, &ns_funcs)?);
            }
        }
    }

    Ok(Program {
        vars,
        funcs,
        prints,
    })
}

use crate::ast;
use anyhow::Context;
use std::collections::HashMap;

pub use ast::Operator;

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
        e: ast::Expr,
        ns_vars: &HashMap<String, usize>,
        ns_funcs: &HashMap<String, (usize, usize)>,
    ) -> anyhow::Result<Self> {
        Ok(match e {
            ast::Expr::Value(v) => Self::Value(v),
            ast::Expr::Variable(s) => {
                Self::Variable(0, *ns_vars.get(&s).context("Using undefined variable.")?)
            }
            ast::Expr::Operation(op, lhs, rhs) => Self::Operation(
                op,
                Box::new(Self::from_ast(*lhs, ns_vars, ns_funcs)?),
                Box::new(Self::from_ast(*rhs, ns_vars, ns_funcs)?),
            ),
            ast::Expr::FuncCall(s, e) => {
                let (id, args) = *ns_funcs.get(&s).context("Using undefined function.")?;
                if e.len() != args {
                    anyhow::bail!("Illegal arguments.");
                }
                Self::FuncCall(
                    id,
                    e.into_iter()
                        .map(|e| Self::from_ast(e, ns_vars, ns_funcs))
                        .collect::<Result<Vec<_>, _>>()?,
                )
            }
            ast::Expr::If(c, t, f) => Self::If(
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

pub fn compile(ast: Vec<ast::Stmt>) -> anyhow::Result<Program> {
    let mut vars = Vec::new();
    let mut ns_vars = HashMap::new();
    let mut funcs = Vec::new();
    let mut ns_funcs = HashMap::new();
    let mut prints = Vec::new();
    for stmt in ast {
        match stmt {
            ast::Stmt::Binding(v, e) => {
                let id = vars.len();
                vars.push(Expr::from_ast(e, &ns_vars, &ns_funcs)?);
                ns_vars.insert(v, id);
            }
            ast::Stmt::Print(e) => {
                prints.push(Expr::from_ast(e, &ns_vars, &ns_funcs)?);
            }
            ast::Stmt::Define(f, a, e) => {
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

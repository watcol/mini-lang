use crate::ast::{Expr, Operator, Stmt};
use anyhow::Context;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Func {
    args: Vec<String>,
    expr: Expr,
}

impl Func {
    fn call<A: AsRef<[Expr]>>(
        &self,
        args: A,
        funcs: &HashMap<String, Self>,
    ) -> anyhow::Result<i32> {
        let mut vars = HashMap::new();
        for (i, arg) in args.as_ref().iter().enumerate() {
            vars.insert(self.args[i].to_owned(), Var::Thunk(arg.clone()));
        }
        eval_expr(&self.expr, &mut vars, funcs)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Var {
    // depth
    Thunk(Expr),
    Cached(i32),
}

impl Var {
    fn get(
        &mut self,
        vars: &mut HashMap<String, Var>,
        funcs: &HashMap<String, Func>,
    ) -> anyhow::Result<i32> {
        Ok(match self {
            Var::Thunk(e) => {
                let val = eval_expr(e, vars, funcs)?;
                *self = Var::Cached(val);
                val
            }
            Var::Cached(v) => *v,
        })
    }
}

pub fn evaluate(ast: Vec<Stmt>) -> anyhow::Result<()> {
    let mut vars = HashMap::new();
    let mut funcs = HashMap::new();
    for stmt in ast {
        match stmt {
            Stmt::Binding(v, e) => {
                vars.insert(v, Var::Thunk(e));
            }
            Stmt::Print(e) => {
                println!("{}", eval_expr(&e, &mut vars, &funcs)?);
            }
            Stmt::Define(f, a, e) => {
                funcs.insert(f, Func { args: a, expr: e });
            }
        }
    }
    Ok(())
}

fn eval_expr(
    expr: &Expr,
    vars: &mut HashMap<String, Var>,
    funcs: &HashMap<String, Func>,
) -> anyhow::Result<i32> {
    Ok(match expr {
        Expr::Value(v) => *v,
        Expr::Variable(v) => {
            let mut var = vars.get(v).context("Use of undefined variable.")?.clone();
            let res = var.get(vars, funcs)?;
            vars.insert(v.to_owned(), var);
            res
        }
        Expr::Operation(op, lhs, rhs) => operation(
            *op,
            eval_expr(lhs, vars, funcs)?,
            eval_expr(rhs, vars, funcs)?,
        )?,
        Expr::FuncCall(f, a) => funcs
            .get(f)
            .context("Use of undefined function.")?
            .call(a, funcs)?,
        Expr::If(c, t, f) => {
            if eval_expr(c, vars, funcs)? != 0 {
                eval_expr(t, vars, funcs)?
            } else {
                eval_expr(f, vars, funcs)?
            }
        }
    })
}

fn operation(op: Operator, lhs: i32, rhs: i32) -> anyhow::Result<i32> {
    Ok(match op {
        Operator::Add => lhs.checked_add(rhs).context("Overflowed addition")?,
        Operator::Sub => lhs.checked_sub(rhs).context("Overflowed subtraction")?,
        Operator::Mul => lhs.checked_mul(rhs).context("Overflowed multiplication")?,
        Operator::Div => lhs
            .checked_div(rhs)
            .context("Overflowed division, or division by zero")?,
        Operator::Rem => lhs
            .checked_rem(rhs)
            .context("Overflowed remainder, or division by zero")?,
        Operator::Gt => {
            if lhs > rhs {
                1
            } else {
                0
            }
        }
        Operator::Ge => {
            if lhs >= rhs {
                1
            } else {
                0
            }
        }
        Operator::Lt => {
            if lhs < rhs {
                1
            } else {
                0
            }
        }
        Operator::Le => {
            if lhs <= rhs {
                1
            } else {
                0
            }
        }
        Operator::Eq => {
            if lhs == rhs {
                1
            } else {
                0
            }
        }
        Operator::Neq => {
            if lhs != rhs {
                1
            } else {
                0
            }
        }
    })
}

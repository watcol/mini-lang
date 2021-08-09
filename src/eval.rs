use crate::ast::{Stmt, Expr, Operator};
use anyhow::Context;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Func {
    args: Vec<String>,
    expr: Expr,
}

impl Func {
    fn call<A: IntoIterator<Item=i32>>(&self, args: A, funcs: &HashMap<String, Self>) -> anyhow::Result<i32> {
        let vars: HashMap<_, _> = self.args.clone().into_iter()
            .zip(args.into_iter())
            .collect();
        eval_expr(&self.expr, &vars, funcs)
    }
}

pub fn evaluate(ast: Vec<Stmt>) -> anyhow::Result<()> {
    let mut vars = HashMap::new();
    let mut funcs = HashMap::new();
    for stmt in ast {
        match stmt {
            Stmt::Binding(v, e) => {
                vars.insert(v, eval_expr(&e, &vars, &funcs)?);
            },
            Stmt::Print(e) => {
                println!("{}", eval_expr(&e, &vars, &funcs)?);
            },
            Stmt::Define(f, a, e) => {
                funcs.insert(f, Func { args: a, expr: e });
            }
        }
    }
    Ok(())
}

fn eval_expr(expr: &Expr, vars: &HashMap<String, i32>, funcs: &HashMap<String, Func>) -> anyhow::Result<i32> {
    Ok(match expr {
        Expr::Value(v) => *v,
        Expr::Variable(v) => *vars.get(v).context("Use of undefined variable.")?,
        Expr::Operation(op, lhs, rhs) => operation(
            *op,
            eval_expr(lhs, vars, funcs)?,
            eval_expr(rhs, vars, funcs)?
        )?,
        Expr::FuncCall(f, a) => funcs.get(f)
            .context("Use of undefined function.")?
            .call(
                a.iter()
                 .map(|e| eval_expr(e, vars, funcs))
                 .collect::<Result<Vec<_>, _>>()?,
                funcs
            )?,
        Expr::If(c, t, f) => if eval_expr(c, vars, funcs)? != 0 {
            eval_expr(t, vars, funcs)?
        } else {
            eval_expr(f, vars, funcs)?
        }
    })
}

fn operation(op: Operator, lhs: i32, rhs: i32) -> anyhow::Result<i32> {
    Ok(match op {
        Operator::Add => lhs.checked_add(rhs).context("Overflowed addition")?,
        Operator::Sub => lhs.checked_sub(rhs).context("Overflowed subtraction")?,
        Operator::Mul => lhs.checked_mul(rhs).context("Overflowed multiplication")?,
        Operator::Div => lhs.checked_div(rhs).context("Overflowed division, or division by zero")?,
        Operator::Rem => lhs.checked_rem(rhs).context("Overflowed remainder, or division by zero")?,
        Operator::Gt => if lhs > rhs { 1 } else { 0 },
        Operator::Ge => if lhs >= rhs { 1 } else { 0 },
        Operator::Lt => if lhs < rhs { 1 } else { 0 },
        Operator::Le => if lhs <= rhs { 1 } else { 0 },
        Operator::Eq => if lhs == rhs { 1 } else { 0 },
        Operator::Neq => if lhs != rhs { 1 } else { 0 },
    })
}

use crate::ir::{Expr, Program};
use crate::Printer;
use super::{Evaluator, operation, NameSpace};

#[derive(Clone, Debug, PartialEq, Eq)]
enum Var {
    Thunk(Expr),
    Cached(i32),
}

impl Var {
    fn get(self, ns: &mut NameSpace<Self>, funcs: &[Expr]) -> anyhow::Result<i32> {
        Ok(match self {
            Self::Thunk(e) => eval_expr(e, ns, funcs)?,
            Self::Cached(i) => i,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LazyEval;

impl Evaluator for LazyEval {
fn evaluate<P: Printer>(&self, ir: Program, printer: &mut P) -> anyhow::Result<()> {
    let Program {
        funcs,
        vars,
        prints,
    } = ir;

    let mut ns = NameSpace::new();
    for var in vars {
        ns.register(Var::Thunk(var));
    }

    for print in prints {
        printer.print(eval_expr(print, &mut ns, &funcs)?)?;
    }
    Ok(())
}
}

fn funccall(
    func: Expr,
    args: Vec<Expr>,
    ns: &mut NameSpace<Var>,
    funcs: &[Expr],
) -> anyhow::Result<i32> {
    let depth = ns.chunk();
    for arg in args {
        ns.register(Var::Thunk(arg));
    }
    let res = eval_expr(func.circulate(depth), ns, funcs)?;
    ns.back();
    Ok(res)
}

fn eval_expr(expr: Expr, ns: &mut NameSpace<Var>, funcs: &[Expr]) -> anyhow::Result<i32> {
    Ok(match expr {
        Expr::Value(v) => v,
        Expr::Variable(depth, id) => {
            let var = ns.borrow(depth, id)?;
            let val = var.get(ns, funcs)?;
            ns.ret(depth, id, Var::Cached(val))?;
            val
        }
        Expr::Operation(op, lhs, rhs) => {
            operation(op, eval_expr(*lhs, ns, funcs)?, eval_expr(*rhs, ns, funcs)?)?
        }
        Expr::FuncCall(f, a) => funccall(funcs[f].clone(), a, ns, funcs)?,
        Expr::If(c, t, f) => {
            if eval_expr(*c, ns, funcs)? != 0 {
                eval_expr(*t, ns, funcs)?
            } else {
                eval_expr(*f, ns, funcs)?
            }
        }
    })
}

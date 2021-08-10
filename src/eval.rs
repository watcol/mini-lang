use crate::ir::{Expr, Program};
use crate::utils::{operation, DataBase};

pub fn evaluate(ir: Program) -> anyhow::Result<()> {
    let Program {
        funcs,
        vars,
        prints,
    } = ir;

    let mut ns = DataBase::new();
    for var in vars {
        let val = eval_expr(var, &mut ns, &funcs)?;
        ns.register(val);
    }

    for print in prints {
        println!("{}", eval_expr(print, &mut ns, &funcs)?);
    }
    Ok(())
}

fn funccall(
    func: Expr,
    args: Vec<Expr>,
    ns: &mut DataBase<i32>,
    funcs: &[Expr],
) -> anyhow::Result<i32> {
    let depth = ns.chunk();
    for arg in args {
        let val = eval_expr(arg, ns, funcs)?;
        ns.register(val);
    }
    let res = eval_expr(func.circulate(depth), ns, funcs)?;
    ns.back();
    Ok(res)
}

fn eval_expr(expr: Expr, ns: &mut DataBase<i32>, funcs: &[Expr]) -> anyhow::Result<i32> {
    Ok(match expr {
        Expr::Value(v) => v,
        Expr::Variable(depth, id) => *ns.get(depth, id)?,
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

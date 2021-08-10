use anyhow::Context;
use crate::ir::Operator;

pub fn operation(op: Operator, lhs: i32, rhs: i32) -> anyhow::Result<i32> {
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

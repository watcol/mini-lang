use crate::ir::Operator;
use crate::MiniResult;

pub fn operation(op: Operator, lhs: i32, rhs: i32) -> MiniResult<i32> {
    Ok(match op {
        Operator::Add => lhs.checked_add(rhs).ok_or("Overflowed addition")?,
        Operator::Sub => lhs.checked_sub(rhs).ok_or("Overflowed subtraction")?,
        Operator::Mul => lhs.checked_mul(rhs).ok_or("Overflowed multiplication")?,
        Operator::Div => lhs
            .checked_div(rhs)
            .ok_or("Overflowed division, or division by zero")?,
        Operator::Rem => lhs
            .checked_rem(rhs)
            .ok_or("Overflowed remainder, or division by zero")?,
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

//! Mini Language
//!
//! This crate provides an interface to use
//! Mini Language as rust library.
//!
//! For the documentation of the language itself, see
//! [README.md](https://github.com/watcol/mini-lang/blob/main/README.md).
mod error;
mod eval;
mod ir;
mod parser;
mod printer;

pub use error::{MiniError, MiniResult};
pub use eval::{EagerEval, Evaluator, LazyEval};
pub use ir::{Expr, Operator, Program};
pub use printer::{Printer, StdPrinter};

/// Execute the code by given evaluator and printer.
pub fn execute<B: AsRef<str>, E: Evaluator, P: Printer>(
    buf: B,
    eval: &E,
    printer: &mut P,
) -> MiniResult<()> {
    let ast = parser::parse(buf)?;
    let ir = ir::compile(ast)?;
    eval.evaluate(ir, printer).map_err(MiniError::from_error)
}

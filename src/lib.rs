//! Mini Language
//!
//! This crate provides an interface to use
//! Mini Language as rust library.
//!
//! For the documentation of the language itself, see
//! [README.md](https://github.com/watcol/mini-lang/blob/main/README.md).
mod ir;
mod parser;
mod eval;
mod printer;
mod error;

pub use printer::{Printer, StdPrinter};
pub use eval::{Evaluator, EagerEval, LazyEval};
pub use error::{MiniResult, MiniError};
pub use ir::{Program, Expr, Operator};

/// Execute the code by given evaluator and printer.
pub fn execute<B: AsRef<str>, E: Evaluator, P: Printer>(buf: B, eval: &E, printer: &mut P) -> MiniResult<()> {
    let ast = parser::parse(buf)?;
    let ir = ir::compile(ast)?;
    eval.evaluate(ir, printer).map_err(MiniError::from_error)
}

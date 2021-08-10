mod ir;
mod parser;
mod eval;
mod printer;
mod error;

pub use printer::{Printer, StdPrinter};
pub use eval::{Evaluator, EagerEval, LazyEval};
pub use error::{MiniResult, MiniError};

pub fn execute<B: AsRef<str>, E: Evaluator, P: Printer>(buf: B, eval: &E, printer: &mut P) -> MiniResult<()> {
    let ast = parser::parse(buf)?;
    let ir = ir::compile(ast)?;
    eval.evaluate(ir, printer).map_err(MiniError::from_error)
}

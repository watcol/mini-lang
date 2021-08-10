mod ir;
mod parser;
mod eval;
mod printer;

pub use printer::{Printer, StdPrinter};
pub use eval::{Evaluator, EagerEval, LazyEval};

pub fn execute<B: AsRef<str>, E: Evaluator, P: Printer>(buf: B, eval: &E, printer: &mut P) -> anyhow::Result<()> {
    let ast = parser::parse(buf)?;
    let ir = ir::compile(ast)?;
    eval.evaluate(ir, printer)
}

mod ir;
mod parser;
mod eval;

pub use eval::{Evaluator, EagerEval, LazyEval};

pub fn execute<B: AsRef<str>, E: Evaluator>(buf: B, eval: E) -> anyhow::Result<()> {
    let ast = parser::parse(buf)?;
    let ir = ir::compile(ast)?;
    eval.evaluate(ir)
}

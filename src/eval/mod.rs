mod eager;
mod lazy;
mod namespace;
mod operation;

pub use eager::EagerEval;
pub use lazy::LazyEval;

use namespace::NameSpace;
use operation::operation;

use crate::ir::Program;
use crate::Printer;

pub trait Evaluator {
    fn evaluate<P: Printer>(&self, ir: Program, printer: &mut P) -> anyhow::Result<()>;
}

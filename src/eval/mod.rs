mod eager;
mod lazy;
mod namespace;
mod operation;

pub use eager::EagerEval;
pub use lazy::LazyEval;

use namespace::NameSpace;
use operation::operation;

use crate::ir::Program;

pub trait Evaluator {
    fn evaluate(&self, ir: Program) -> anyhow::Result<()>;
}

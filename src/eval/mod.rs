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
    type Err: std::error::Error + 'static;
    fn evaluate<P: Printer>(&self, ir: Program, printer: &mut P) -> Result<(), Self::Err>;
}

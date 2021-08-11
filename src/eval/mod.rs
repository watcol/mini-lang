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

/// The trait to provide kinds of evaluation (or compilation).
pub trait Evaluator {
    /// The error type that evaluator will provide.
    type Err: std::error::Error + 'static;
    /// Evaluate `Program` and print expression by `printer`.
    fn evaluate<P: Printer>(&self, ir: Program, printer: &mut P) -> Result<(), Self::Err>;
}

use std::fmt;

/// The printer to print evaluated value.
pub trait Printer {
    /// The error type that printer will provide.
    type Err: std::error::Error + 'static;
    /// Print integer value.
    fn print(&mut self, v: i32) -> Result<(), Self::Err>;
}

/// The default printer implementation which prints to stdout.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct StdPrinter;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct EmptyError;

impl fmt::Display for EmptyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EmptyError")
    }
}

impl std::error::Error for EmptyError {}

impl Printer for StdPrinter {
    type Err = EmptyError;
    fn print(&mut self, v: i32) -> Result<(), Self::Err> {
        println!("{}", v);
        Ok(())
    }
}

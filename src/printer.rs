use std::fmt;

pub trait Printer {
    type Err: std::error::Error + 'static;
    fn print(&mut self, v: i32) -> Result<(), Self::Err>;
}

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

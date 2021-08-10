pub trait Printer {
    fn print(&mut self, v: i32) -> anyhow::Result<()>;
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct StdPrinter;

impl Printer for StdPrinter {
    fn print(&mut self, v: i32) -> anyhow::Result<()> {
        println!("{}", v);
        Ok(())
    }
}

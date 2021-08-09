mod parser;
mod ast;

use std::env::args;
use std::fs::File;
use std::io::{stdin, Read};

fn main() -> anyhow::Result<()> {
    let path = args().nth(1);
    let mut buf = String::new();
    match path {
        Some(path) => File::open(path)?.read_to_string(&mut buf)?,
        None => stdin().read_to_string(&mut buf)?,
    };

    println!("{:?}", parser::parse(&buf)?);
    Ok(())
}

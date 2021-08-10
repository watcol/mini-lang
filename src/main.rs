mod eval;
mod ir;
mod parser;

use std::fs::File;
use std::io::{stdin, Read};
use structopt::StructOpt;
use eval::{Evaluator, EagerEval, LazyEval};

#[derive(Debug, StructOpt)]
#[structopt(author, about)]
struct Opt {
    #[structopt(short, long, help = "Enables lazy evaluation.")]
    lazy: bool,

    #[structopt(name = "FILE", help = "The input file.")]
    path: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();
    let mut buf = String::new();
    match opt.path {
        Some(path) => File::open(path)?.read_to_string(&mut buf)?,
        None => stdin().read_to_string(&mut buf)?,
    };

    let ast = parser::parse(&buf)?;
    let ir = ir::compile(ast)?;
    if opt.lazy {
        LazyEval::evaluate(ir)?;
    } else {
        EagerEval::evaluate(ir)?;
    }
    Ok(())
}

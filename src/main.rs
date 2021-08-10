use mini_lang::{StdPrinter, EagerEval, LazyEval, execute};
use structopt::StructOpt;
use std::fs::File;
use std::io::{stdin, Read};

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

    if opt.lazy {
        execute(&buf, &LazyEval, &mut StdPrinter)?;
    } else {
        execute(&buf, &EagerEval, &mut StdPrinter)?;
    }
    Ok(())
}

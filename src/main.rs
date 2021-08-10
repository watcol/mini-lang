use mini_lang::{StdPrinter, EagerEval, LazyEval, execute, MiniError, MiniResult};
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

fn main() {
    inner_main().unwrap_or_else(|e| {
        println!("{}", e);
        std::process::exit(1);
    })
}

fn inner_main() -> MiniResult<()> {
    let opt = Opt::from_args();
    let mut buf = String::new();
    match opt.path {
        Some(path) => File::open(path).map_err(MiniError::from_error)?.read_to_string(&mut buf).map_err(MiniError::from_error)?,
        None => stdin().read_to_string(&mut buf).map_err(MiniError::from_error)?,
    };

    if opt.lazy {
        execute(&buf, &LazyEval, &mut StdPrinter)?;
    } else {
        execute(&buf, &EagerEval, &mut StdPrinter)?;
    }
    Ok(())
}

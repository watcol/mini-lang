#![allow(deprecated)]

use criterion::{criterion_group, criterion_main, Criterion, ParameterizedBenchmark};
use std::fmt;

use mini_lang::{execute, EagerEval, Evaluator, LazyEval, Printer};

struct NopPrinter;

#[derive(Debug)]
struct EmptyError;

impl fmt::Display for EmptyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "empty")
    }
}

impl std::error::Error for EmptyError {}

impl Printer for NopPrinter {
    type Err = EmptyError;
    fn print(&mut self, _v: i32) -> Result<(), EmptyError> {
        Ok(())
    }
}

fn exec_tarai<E: Evaluator>(n: i32, eval: &E) {
    let program = format!(
        indoc::indoc! {"
        def tarai(x, y, z) = \\
            if x <= y \\
                then y \\
        else tarai(tarai(x-1, y, z), tarai(y-1, z, x), tarai(z-1, x, y))

        print tarai({}, {}, 0)
    "},
        n * 2,
        n
    );
    execute(&program, eval, &mut NopPrinter).unwrap();
}

fn tarai(c: &mut Criterion) {
    c.bench(
        "tarai",
        ParameterizedBenchmark::new(
            "eager",
            |b, i| b.iter(|| exec_tarai(*i, &EagerEval)),
            vec![1, 2, 3, 4, 5],
        )
        .with_function("lazy", |b, i| b.iter(|| exec_tarai(*i, &LazyEval))),
    );
}

criterion_group!(taraibench, tarai);
criterion_main!(taraibench);

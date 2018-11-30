#[macro_use]
extern crate criterion;
extern crate kenken;

use kenken::solver::solve;

use criterion::Criterion;

fn criterion_benchmark(c: &mut Criterion) {
    let (size_5, cons_5) = kenken::get_input("puzzle_5.dat");
    c.bench_function("solve 5", move |b| b.iter(|| solve(size_5, &cons_5)));
    let (size_6, cons_6) = kenken::get_input("puzzle_6.dat");
    c.bench_function("solve 6", move |b| b.iter(|| solve(size_6, &cons_6)));
    let (size_7, cons_7) = kenken::get_input("puzzle.dat");
    c.bench_function("solve 7", move |b| b.iter(|| solve(size_7, &cons_7)));
    c.bench_function("solve 7 full", |b| b.iter(|| kenken::solve("puzzle.dat")));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

use criterion_truncation_bug::*;
#[macro_use]
extern crate criterion;

use criterion::Criterion;

fn criterion_benchmark_fast(c: &mut Criterion) {
    c.bench_function(
        "a_bench_with_an_obnoxiously_ridiculously_amazingly_stupidly_long_name_fast",
        |b| b.iter(|| fast()),
    );
}

fn criterion_benchmark_slow(c: &mut Criterion) {
    c.bench_function(
        "a_bench_with_an_obnoxiously_ridiculously_amazingly_stupidly_long_name_slow",
        |b| b.iter(|| slow()),
    );
}

criterion_group!(benches, criterion_benchmark_fast, criterion_benchmark_slow);
criterion_main!(benches);

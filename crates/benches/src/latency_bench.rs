//! latency_bench.rs
//! Benchmarking file for the `benches` crate using Criterion.

use criterion::{criterion_group, criterion_main, Criterion};
use benches::example_function;

/// Benchmark for the `example_function`.
pub fn benchmark_example_function(c: &mut Criterion) {
    c.bench_function("example_function", |b| {
        b.iter(|| example_function());
    });
}

// Group benchmarks and define the main entry point for Criterion.
criterion_group!(benches, benchmark_example_function);
criterion_main!(benches);

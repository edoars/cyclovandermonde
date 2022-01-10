use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use cyclovander::tr_h;
use pprof::criterion::{Output, PProfProfiler};

fn bench_small_factors(c: &mut Criterion) {
    let mut group = c.benchmark_group("trace_small_factors");

    for s in &[3, 15, 105, 1155] {
        group.bench_with_input(BenchmarkId::from_parameter(s), s, |b, s| {
            b.iter(|| tr_h(black_box(*s)))
        });
    }
}

fn bench_primes(c: &mut Criterion) {
    let mut group = c.benchmark_group("trace_primes");

    for s in &[13, 103, 1153] {
        group.bench_with_input(BenchmarkId::from_parameter(s), s, |b, s| {
            b.iter(|| tr_h(black_box(*s)))
        });
    }
}

criterion_group! {
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets = bench_small_factors, bench_primes
}
criterion_main!(benches);

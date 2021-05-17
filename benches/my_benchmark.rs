use criterion::{black_box, criterion_group, criterion_main, Criterion};
use criterion_gettingstarted::{fibonacci_slow,fibonacci_fast};


pub fn criterion_bench_fib1(c: &mut Criterion) {
    c.bench_function("fib20-slow", |b| b.iter(|| fibonacci_slow(black_box(20))));
}

pub fn criterion_bench_fib2(c: &mut Criterion) {
    c.bench_function("fib20-fast", |b| b.iter(|| fibonacci_fast(black_box(20))));
}

criterion_group!(benches, criterion_bench_fib1, criterion_bench_fib2 );
criterion_main!(benches);

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use criterion_gettingstarted::fibonacci;
use criterion_gettingstarted::fibonacci2;


pub fn criterion_bench_fib1(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

pub fn criterion_bench_fib2(c: &mut Criterion) {
    c.bench_function("fib 20 - fib2", |b| b.iter(|| fibonacci2(black_box(20))));
}

criterion_group!(benches, criterion_bench_fib1, criterion_bench_fib2 );
criterion_main!(benches);

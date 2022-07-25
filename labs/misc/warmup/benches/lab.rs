use criterion::{black_box, criterion_group, criterion_main, Criterion};

use warmup::solution;

fn bench1(c: &mut Criterion) {
    // problem: count sum of all the numbers up to N
    const N: usize = 1000;
    let mut arr = [0i32; N];
    for i in 0..N {
        arr[i] = i as i32 + 1i32;
    }

    let mut result = 0;

    // benchmark
    c.bench_function("solution 1000", |b| {
        b.iter(|| result = solution(black_box(&arr), N))
    });
}

criterion_group!(benches, bench1);
criterion_main!(benches);

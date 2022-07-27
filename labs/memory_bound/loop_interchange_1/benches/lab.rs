#![feature(bench_black_box)]

use criterion::{criterion_group, criterion_main, Criterion};

use loop_interchange_1::{init, power, zero, N};

fn bench1(c: &mut Criterion) {
    let mut matrix_a = Box::new([[0.0f32; N]; N]);
    init(&mut matrix_a);
    let mut matrix_b = Box::new([[0.0f32; N]; N]);
    zero(&mut matrix_b);

    c.bench_function("lab", |b| {
        b.iter(|| {
            matrix_b = power(&matrix_a, 2021);
            std::hint::black_box(&mut matrix_b);
        });
    });
}

criterion_group!(benches, bench1);
criterion_main!(benches);

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use scratch::{a_rec, a_iter};

fn bench_iterative(c: &mut Criterion) {
    c.bench_function("A(4, 1) iterative", |b| {
        b.iter(|| a_iter(black_box(4), black_box(1)))
    });
}
fn bench_recursive(c: &mut Criterion) {
    c.bench_function("A(4, 1) recursive", |b| {
        b.iter(|| a_rec(black_box(4), black_box(1)))
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = bench_iterative, bench_recursive
}

criterion_main!(benches);

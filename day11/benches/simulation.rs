use criterion::{criterion_group, criterion_main, Criterion};

use day11::{data::create_factory, simulate};

pub fn criterion_benchmark(c: &mut Criterion) {
    let layout = create_factory();

    c.bench_function("simulate", |b| {
        b.iter(|| simulate(&layout, 5, std::isize::MAX))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

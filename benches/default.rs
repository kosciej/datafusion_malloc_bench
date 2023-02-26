use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    datafusion_malloc_bench::select_all_benchmark(c, "default");
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

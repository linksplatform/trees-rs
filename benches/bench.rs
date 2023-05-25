use {
    criterion::{criterion_group, criterion_main, Criterion, Throughput},
};

pub fn bench(c: &mut Criterion) {
    const MAGIC: usize = 1_000_000;

}

criterion_group!(benches, bench);
criterion_main!(benches);

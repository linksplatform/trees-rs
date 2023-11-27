use {
    criterion::{criterion_group, criterion_main, Criterion, Throughput},
    platform_trees::{BTree, New, OldStore},
    std::num::NonZeroUsize,
};

pub fn bench(c: &mut Criterion) {
    const MAGIC: usize = 100_000;

    c.benchmark_group("trees")
        .throughput(Throughput::Elements(MAGIC as u64))
        .bench_function("old", |b| {
            let mut place = OldStore::<usize>::make(MAGIC);
            b.iter(|| {
                let mut root = None;

                for i in 1..MAGIC {
                    place._attach(&mut root, i)
                }
                for i in 1..MAGIC {
                    place._detach(&mut root, i);
                }
                place.reset();
            })
        })
        .bench_function("new", |b| {
            use platform_trees::new::NoRecur;

            let mut place = New::new(MAGIC);
            b.iter(|| unsafe {
                let mut root = None;

                for i in 1..MAGIC {
                    root = place.attach(root, NonZeroUsize::new(i).unwrap_unchecked());
                }
                for i in 1..MAGIC {
                    root = place.detach(root, NonZeroUsize::new(i).unwrap_unchecked());
                }
                place.fill(Default::default())
            })
        });
}

criterion_group!(benches, bench);
criterion_main!(benches);

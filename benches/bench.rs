use {
    criterion::{criterion_group, criterion_main, Criterion, Throughput},
    platform_trees::{BTree, New, NewV2, OldStore, Store},
};

pub fn bench(c: &mut Criterion) {
    const MAGIC: usize = 1_000_000;

    c.benchmark_group("trees")
        .throughput(Throughput::Elements(MAGIC as u64))
        .throughput(Throughput::BytesDecimal(MAGIC as u64 * 24))
        .bench_function("old", |b| {
            let mut place = OldStore::<usize>::new(MAGIC + 1);
            b.iter(|| {
                let mut root = None;

                for i in 2..=MAGIC {
                    place._attach(&mut root, i)
                }
                for i in 2..=MAGIC {
                    place._detach(&mut root, i);
                }
                place.reset();
            })
        })
        .bench_function("new", |b| {
            let mut place = New(Store::<usize>::new(MAGIC));
            b.iter(|| {
                let mut root = None;

                for i in 2..=MAGIC {
                    place._attach(&mut root, i);
                }
                place.reset();
            })
        })
        .bench_function("new-v2", |b| {
            let mut place = NewV2(Store::<usize>::new(MAGIC));
            b.iter(|| {
                let mut root = None;

                for i in 2..=MAGIC {
                    place._attach(&mut root, i);
                }
                for i in 2..=MAGIC {
                    place._detach(&mut root, i);
                }
                place.reset();
            })
        });
}

criterion_group!(benches, bench);
criterion_main!(benches);

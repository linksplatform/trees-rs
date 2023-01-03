#![feature(default_free_fn)]

use criterion::{criterion_group, criterion_main, Criterion};
use inner::{BTree, New, NewV2, OldStore, Store};
use std::default::default;

mod inner;

fn reset<T: Default>(slice: &mut [T]) {
    for item in slice {
        *item = default();
    }
}

const MAGIC: usize = 10_000;

pub fn bench(c: &mut Criterion) {
    c.bench_function("old", |b| {
        let mut place = OldStore::<usize>::new(MAGIC);
        b.iter(|| {
            reset(&mut place);
            let mut root = None;
            for i in 2..=MAGIC {
                place.add(&mut root, i)
            }
        })
    });

    c.bench_function("new", |b| {
        let mut place = New(Store::<usize>::new(MAGIC));
        b.iter(|| {
            reset(&mut place);
            let mut root = None;
            for i in 2..=MAGIC {
                place.add(&mut root, i);
            }
        })
    });

    c.bench_function("new-v2", |b| {
        let mut place = NewV2(Store::<usize>::new(MAGIC));
        b.iter(|| {
            reset(&mut place);
            let mut root = None;
            for i in 2..=MAGIC {
                place.add(&mut root, i);
            }
        })
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);

#![feature(default_free_fn)]

use criterion::{criterion_group, criterion_main, Criterion};
use trees_bench::{Basic, Storage, Storage2, Storage3};

mod trees_bench;

pub fn old_trees(c: &mut Criterion) {
    c.bench_function("create_nodes_old", |b| {
        b.iter(|| {
            let mut tree = Storage::<usize>::new(100);
            let mut root = None;
            for node in 2..=100 {
                tree.add(&mut root, node)
            }
        })
    });
}

pub fn new_trees(c: &mut Criterion) {
    c.bench_function("create_nodes_new", |b| {
        b.iter(|| {
            let mut tree = Storage2::<usize>::new(100);
            let mut root = None;
            for node in 2..=100 {
                tree.add(&mut root, node);
            }
        })
    });
}

pub fn new_new_trees(c: &mut Criterion) {
    c.bench_function("create_nodes_new_new", |b| {
        b.iter(|| {
            let mut tree = Storage3::<usize>::new(100);
            let mut root = None;
            for node in 2..=100 {
                tree.add(&mut root, node);
            }
        })
    });
}

criterion_group!(benches, old_trees, new_trees, new_new_trees);
criterion_main!(benches);

use {
    platform_trees::{inner, BTree, OldStore, Store},
    std::collections::HashSet,
};

use proptest::prelude::*;

trait QuickTree {
    type Output: BTree<Item = usize>;

    fn new(len: usize) -> Self::Output;
}

macro_rules! quick_impl {
    ($($name:ident | $ty:ty => $expr:expr)*) => {$(
        struct $name;

        impl QuickTree for $name {
            type Output = $ty;

            fn new(len: usize) -> Self::Output {
                ($expr)(len + 1)
            }
        })*
    };
}

quick_impl!(
    Old | OldStore<usize> => |len| OldStore::new(len)
    New | inner::New<usize> => |len| inner::New(Store::new(len))
    NewV2 | inner::NewV2<usize> => |len| inner::NewV2(Store::new(len))
);

prop_compose! {
    fn seq_inner(vec: Vec<usize>)
    (set in prop::collection::hash_set(1..vec.len(), 0..50)) -> HashSet<usize> {
        set
    }
}

prop_compose! {
    fn seq_strategy()
        (len in 2..100_usize)
        (len in Just(len), set in prop::collection::hash_set(1..len, 0..100))
    -> (HashSet<usize>, usize) {
       (set, len)
    }
}

fn inner<Tree: QuickTree>((vec, len): (HashSet<usize>, usize)) {
    let mut store = Tree::new(len);
    let mut root = None;
    for item in vec.iter().copied() {
        store._attach(&mut root, item);
        assert!(store.is_contains(root.unwrap(), item));
    }

    for item in vec {
        store._detach(&mut root, item);
        if let Some(root) = root.clone() {
            assert!(!store.is_contains(root, item));
        }
    }
}

use proptest::test_runner::FileFailurePersistence;

proptest! {
    #![proptest_config(ProptestConfig {
        failure_persistence: Some(Box::new(FileFailurePersistence::WithSource("regressions"))),
        ..Default::default()
    })]

    #[test]
    fn prop_old(seq in seq_strategy()) {
        inner::<Old>(seq)
    }

    #[test]
    fn prop_new(seq in seq_strategy()) {
        // fixme: does not support `detach`
        // inner::<New>(seq)
    }

    #[test]
    fn prop_new_v2(seq in seq_strategy()) {
        inner::<NewV2>(seq)
    }
}

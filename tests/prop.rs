use {
    std::collections::HashSet,
};

use proptest::prelude::*;

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

use proptest::test_runner::FileFailurePersistence;

proptest! {
    #![proptest_config(ProptestConfig {
        failure_persistence: Some(Box::new(FileFailurePersistence::WithSource("regressions"))),
        ..Default::default()
    })]

    #[test]
    fn prop_new(seq in seq_strategy()) {
        // fixme: does not support `detach`
        // inner::<New>(seq)
    }

}

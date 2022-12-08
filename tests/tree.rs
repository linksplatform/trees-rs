#![feature(default_free_fn)]

use crate::dummy::DummySzb;
use platform_trees::{NoRecurSzbTree, SzbTree};

mod dummy;

#[test]
fn basic() {
    let mut tree = DummySzb::<usize>::new(234);

    unsafe {
        let mut root = 1;

        tree.attach(&mut root, 2);
        tree.attach(&mut root, 3);
        tree.attach(&mut root, 4);

        assert!(tree.contains(3, root));
        assert!(!tree.contains(5, root));

        println!("{:?}", tree);
    }
}

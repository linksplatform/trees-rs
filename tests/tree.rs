#![feature(default_free_fn)]

use crate::dummy::{Dummy, DummySzb, Node2};
use platform_trees::{new, new_v2, NoRecurSzbTree, SzbTree};

mod dummy;

#[test]
fn basic() {
    let mut tree = DummySzb::<usize>::new(5);

    unsafe {
        let mut root = 0;

        tree.attach(&mut root, 2);
        tree.attach(&mut root, 3);
        tree.attach(&mut root, 4);

        assert!(tree.contains(3, root));
        assert!(!tree.contains(5, root));

        println!("{root:?}");
        println!("{tree:#?}");
    }
}

#[test]
fn new() {
    let mut tree: Vec<_> = (0..5).map(|_| Node2::<usize>::default()).collect();
    let slice = tree.as_mut_slice();

    let mut root = None;

    use new::{NoRecur, Tree};

    root = Dummy::attach(slice, root, 2);
    root = Dummy::attach(slice, root, 3);
    root = Dummy::attach(slice, root, 4);

    let root = root.unwrap();

    assert!(Dummy::is_contains(slice, root, 3));
    assert!(!Dummy::is_contains(slice, root, 5));

    println!("{root:?}");
    println!("{tree:#?}");
}

#[test]
fn new_v2() {
    let mut tree: Vec<_> = (0..5).map(|_| Node2::<usize>::default()).collect();
    let slice = tree.as_mut_slice();

    let mut root = None;

    use new_v2::{NoRecur, Tree};

    root = Dummy::attach(slice, root, 2);
    root = Dummy::attach(slice, root, 3);
    root = Dummy::attach(slice, root, 4);

    let root = root.unwrap();

    assert!(Dummy::is_contains(slice, root, 3));
    assert!(!Dummy::is_contains(slice, root, 5));

    println!("{root:?}");
    println!("{tree:#?}");
}

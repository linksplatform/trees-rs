#![feature(default_free_fn)]

use crate::dummy::{Dummy, Dummy2, DummySzb, Node2};
use platform_trees::{NewNewNoRecur, NewNoRecur, NewTree, NoRecurSzbTree, SzbTree};

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
        println!("----------");
        assert!(!tree.contains(5, root));

        println!("{:?}", tree);
    }
}

#[test]
fn new() {
    let mut tree: Vec<_> = (0..30).map(|_| Node2::<usize>::default()).collect();
    let slice = tree.as_mut_slice();

    let mut root = None;

    root = Dummy2::attach(slice, root, 2);
    root = Dummy2::attach(slice, root, 3);
    root = Dummy2::attach(slice, root, 4);

    let root = root.unwrap();

    assert!(Dummy::is_contains(slice, root, 3));
    assert!(!Dummy::is_contains(slice, root, 5));

    println!("{:?}", root);
    println!("{:#?}", tree);
}

#[test]
fn new2() {
    let mut tree: Vec<_> = (0..30).map(|_| Node2::<usize>::default()).collect();
    let slice = tree.as_mut_slice();

    let mut root = None;

    root = Dummy2::attach(slice, root, 2);
    root = Dummy2::attach(slice, root, 3);
    root = Dummy2::attach(slice, root, 4);

    let root = root.unwrap();

    assert!(Dummy::is_contains(slice, root, 3));
    assert!(!Dummy::is_contains(slice, root, 5));

    println!("{:?}", root);
    println!("{:#?}", tree);
}

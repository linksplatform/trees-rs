#![feature(default_free_fn)]

use crate::dummy::{Dummy, DummySzb, Node2, Simplifier};
use platform_trees::{new, new_v2, NoRecurSzbTree, SzbTree};
use quickcheck::quickcheck;
use std::collections::HashSet;
use std::default::default;

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

quickcheck! {
    fn works(nodes: Simplifier<usize>) -> () {
        assert!(nodes.0.len() != 0);
        {
            let mut tree = DummySzb::<usize>::new(nodes.0.len());
            let mut root = 0;
            unsafe {
                for node in &nodes.0 {
                    tree.attach(&mut root, *node);
                    assert!(tree.contains(*node, root));
                }
                for node in &nodes.0 {
                    tree.detach(&mut root, *node);
                    assert!(!tree.contains(*node, root));
                }
            }
        }
        {
            use new::{NoRecur,Tree};
            let mut root = None;
            let mut tree: Vec<_> = (0..nodes.0.len()).map(|_| Node2::<usize>::default()).collect();
            for node in &nodes.0 {
                root = Dummy::attach(tree.as_mut_slice(), root, *node);
                assert!(Dummy::is_contains(tree.as_slice(), root.unwrap(), *node));
            }
            //TODO: detach method has not been implemented yet
            //for node in &nodes.0 {
            //    root = Dummy::detach(tree.as_mut_slice(), root, *node)
            //    assert!(!Dummy::is_contains(tree.as_slice(), root, *node));
            //}
        }
        {
            use new_v2::{NoRecur, Tree};
            let mut root = None;
            let mut tree: Vec<_> = (0..nodes.0.len()).map(|_| Node2::<usize>::default()).collect();
            for node in &nodes.0 {
                root = Dummy::attach(tree.as_mut_slice(), root, *node);
                assert!(Dummy::is_contains(tree.as_slice(), root.unwrap(), *node));
            }
            //TODO: detach method has not been implemented yet
            //for node in &nodes.0 {
            //    root = Dummy::detach(tree.as_mut_slice(), root, *node)
            //    assert!(!Dummy::is_contains(tree.as_slice(), root.unwrap(), *node));
            //}
        }
        ()
    }
}

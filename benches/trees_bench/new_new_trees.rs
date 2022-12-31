use crate::trees_bench::{Basic, Node2, Storage2};
use platform_data::LinkType;
use platform_trees::{NewNewNoRecur, NewNewTree, NewNoRecur, Node};
use std::default::default;

pub struct Storage3<T> {
    storage: Vec<Node2<T>>,
}

impl<T: LinkType> Storage3<T> {
    pub fn new(size: usize) -> Storage3<T> {
        Self {
            storage: (0..size).map(|_| default()).collect(),
        }
    }
}

impl<T: LinkType + From<usize>> NewNewTree for Storage3<T> {
    type Item = Node2<T>;

    fn get(item: &Self::Item) -> Node {
        Node {
            size: item.size.as_usize(),
            left: item.left.map(T::as_usize),
            right: item.right.map(T::as_usize),
        }
    }

    fn set(item: &mut Self::Item, val: Node) {
        item.size = val.size.into();
        item.left = val.left.map(Into::into);
        item.right = val.right.map(Into::into);
    }

    fn is_left_of(_: &[Self::Item], first: usize, second: usize) -> bool {
        first < second
    }
}

impl<T: LinkType + From<usize>> NewNewNoRecur for Storage3<T> {}

impl<T: LinkType + From<usize>> Basic<T> for Storage3<T> {
    type Node = T;

    fn add(&mut self, root: &mut Option<Self::Node>, node: Self::Node) {
        *root = Storage3::attach(
            self.storage.as_mut_slice(),
            root.map(T::as_usize),
            node.as_usize(),
        )
        .map(T::from)
    }

    fn remove(&mut self, root: &mut Option<Self::Node>, node: Self::Node) {
        *root = Storage2::detach(self.storage.as_mut_slice(), *root, node)
    }
}

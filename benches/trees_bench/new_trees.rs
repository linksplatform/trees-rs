use std::default::default;

use platform_data::LinkType;
use tap::Pipe;

use platform_trees::{NewNoRecur, NewTree};

use crate::trees_bench::Basic;

#[derive(Default)]
pub struct Node2<T> {
    pub(crate) size: T,
    pub(crate) left: Option<T>,
    pub(crate) right: Option<T>,
}

pub struct Storage2<T> {
    storage: Vec<Node2<T>>,
}

impl<T: LinkType> Storage2<T> {
    pub fn new(size: usize) -> Storage2<T> {
        Self {
            storage: (0..size).map(|_| default()).collect(),
        }
    }
}

impl<T: LinkType> NewTree<T> for Storage2<T> {
    type Item = Node2<T>;

    fn size(slice: &[Self::Item], idx: T) -> Option<T> {
        slice.get(idx.as_usize())?.size.pipe(Some)
    }

    fn left(slice: &[Self::Item], idx: T) -> Option<T> {
        slice.get(idx.as_usize())?.left
    }

    fn right(slice: &[Self::Item], idx: T) -> Option<T> {
        slice.get(idx.as_usize())?.right
    }

    fn set_size(slice: &mut [Self::Item], idx: T, value: T) {
        if let Some(node) = slice.get_mut(idx.as_usize()) {
            node.size = value
        }
    }

    fn set_left(slice: &mut [Self::Item], idx: T, value: Option<T>) {
        if let Some(node) = slice.get_mut(idx.as_usize()) {
            node.left = value
        }
    }

    fn set_right(slice: &mut [Self::Item], idx: T, value: Option<T>) {
        if let Some(node) = slice.get_mut(idx.as_usize()) {
            node.right = value
        }
    }

    fn is_left_of(_: &[Self::Item], first: T, second: T) -> bool {
        first < second
    }

    fn is_right_of(_: &[Self::Item], first: T, second: T) -> bool {
        first > second
    }
}

impl<T: LinkType> NewNoRecur<T> for Storage2<T> {}

impl<T: LinkType> Basic<T> for Storage2<T> {
    type Node = T;

    fn add(&mut self, root: &mut Option<Self::Node>, node: Self::Node) {
        *root = Storage2::attach(self.storage.as_mut_slice(), *root, node)
    }

    fn remove(&mut self, root: &mut Option<Self::Node>, node: Self::Node) {
        *root = Storage2::detach(self.storage.as_mut_slice(), *root, node)
    }
}

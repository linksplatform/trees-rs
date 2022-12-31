use crate::trees_bench::Basic;
use platform_data::LinkType;
use platform_trees::{NoRecurSzbTree, SzbTree};
use std::default::default;

#[derive(Default)]
struct Node<T> {
    size: T,
    left: T,
    right: T,
}

pub struct Storage<T> {
    store: Vec<Node<T>>,
}

impl<T: LinkType> Storage<T> {
    pub fn new(size: usize) -> Storage<T> {
        Self {
            store: (0..size).map(|_| default()).collect(),
        }
    }
}

impl<T: LinkType> SzbTree<T> for Storage<T> {
    unsafe fn get_mut_left_reference(&mut self, node: T) -> *mut T {
        &mut self.store.get_mut(node.as_usize()).unwrap().left
    }

    unsafe fn get_mut_right_reference(&mut self, node: T) -> *mut T {
        &mut self.store.get_mut(node.as_usize()).unwrap().right
    }

    unsafe fn get_left_reference(&self, node: T) -> *const T {
        &self.store.get(node.as_usize()).unwrap().left
    }

    unsafe fn get_right_reference(&self, node: T) -> *const T {
        &self.store.get(node.as_usize()).unwrap().right
    }

    unsafe fn get_left(&self, node: T) -> T {
        self.store.get(node.as_usize()).unwrap().left
    }

    unsafe fn get_right(&self, node: T) -> T {
        self.store.get(node.as_usize()).unwrap().right
    }

    unsafe fn get_size(&self, node: T) -> T {
        self.store.get(node.as_usize()).unwrap().size
    }

    unsafe fn set_left(&mut self, node: T, left: T) {
        if let Some(value) = self.store.get_mut(node.as_usize()) {
            value.left = left
        }
    }

    unsafe fn set_right(&mut self, node: T, right: T) {
        if let Some(value) = self.store.get_mut(node.as_usize()) {
            value.right = right
        }
    }

    unsafe fn set_size(&mut self, node: T, size: T) {
        if let Some(value) = self.store.get_mut(node.as_usize()) {
            value.size = size
        }
    }

    unsafe fn first_is_to_the_left_of_second(&self, first: T, second: T) -> bool {
        first < second
    }

    unsafe fn first_is_to_the_right_of_second(&self, first: T, second: T) -> bool {
        first > second
    }
}

impl<T: LinkType> NoRecurSzbTree<T> for Storage<T> {}

impl<T: LinkType> Basic<T> for Storage<T> {
    type Node = T;

    fn add(&mut self, root: &mut Option<Self::Node>, node: Self::Node) {
        if let Some(root) = root {
            unsafe { self.attach(root, node) }
        } else {
            *root = Some(T::one());
            unsafe { self.attach(root.as_mut().unwrap(), node) }
        }
    }

    fn remove(&mut self, root: &mut Option<Self::Node>, node: Self::Node) {
        if let Some(root) = root {
            unsafe { self.detach(root, node) }
        }
    }
}

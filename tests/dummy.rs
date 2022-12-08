use platform_data::LinkType;
use platform_trees::{NoRecurSzbTree, SzbTree};
use std::default::default;

#[derive(Debug, Default)]
struct Node<T> {
    pub size: T,
    pub left: T,
    pub right: T,
}

#[derive(Debug)]
pub struct DummySzb<T> {
    place: Vec<Node<T>>,
}

impl<T: LinkType> DummySzb<T> {
    pub fn new(size: usize) -> Self {
        Self {
            place: (0..size).map(|_| default()).collect(),
        }
    }

    fn get(&self, index: T) -> Option<&Node<T>> {
        self.place.get(index.as_usize())
    }

    fn get_mut(&mut self, index: T) -> Option<&mut Node<T>> {
        self.place.get_mut(index.as_usize())
    }
}

impl<T: LinkType> SzbTree<T> for DummySzb<T> {
    unsafe fn get_mut_left_reference(&mut self, node: T) -> *mut T {
        &mut self.get_mut(node).unwrap().left
    }

    unsafe fn get_mut_right_reference(&mut self, node: T) -> *mut T {
        &mut self.get_mut(node).unwrap().right
    }

    unsafe fn get_left_reference(&self, node: T) -> *const T {
        &self.get(node).unwrap().left
    }

    unsafe fn get_right_reference(&self, node: T) -> *const T {
        &self.get(node).unwrap().right
    }

    unsafe fn get_left(&self, node: T) -> T {
        self.get(node).unwrap().left
    }

    unsafe fn get_right(&self, node: T) -> T {
        self.get(node).unwrap().right
    }

    unsafe fn get_size(&self, node: T) -> T {
        self.get(node).unwrap().size
    }

    unsafe fn set_left(&mut self, node: T, left: T) {
        self.get_mut(node).unwrap().left = left;
    }

    unsafe fn set_right(&mut self, node: T, right: T) {
        self.get_mut(node).unwrap().right = right;
    }

    unsafe fn set_size(&mut self, node: T, size: T) {
        self.get_mut(node).unwrap().size = size;
    }

    unsafe fn first_is_to_the_left_of_second(&self, first: T, second: T) -> bool {
        first < second
    }

    unsafe fn first_is_to_the_right_of_second(&self, first: T, second: T) -> bool {
        first > second
    }
}

impl<T: LinkType> NoRecurSzbTree<T> for DummySzb<T> {}

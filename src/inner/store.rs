use {
    funty::Fundamental,
    platform_data::LinkType,
    std::{
        default::default,
        fmt::Debug,
        ops::{Deref, DerefMut},
    },
};

use {
    super::BTree,
    crate::{
        new::{self, NoRecur},
        LinkType as Bridge, NoRecurSzbTree, SzbTree,
    },
    tap::Pipe,
};

#[derive(Debug, Default, Copy, Clone)]
pub struct Node<T> {
    pub size: T,
    pub left: Option<T>,
    pub right: Option<T>,
}

macro_rules! deref_derive {
    (
        $(
            $vis:vis struct $name:ident$(
                <$($arg:ident),+ $(,)?>
            )?($target:ty);
        )+
    ) => {
        $(
            $vis struct $name$(
                <$($arg),+>
            )?(pub $target);

            impl$(<$($arg),+>)? Deref for $name$(<$($arg),+>)? {
                type Target = $target;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            impl$(<$($arg),+>)? DerefMut for $name$(<$($arg),+>)? {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.0
                }
            }
        )+
    };
}

#[derive(Debug, Default, Copy, Clone)]
pub struct OldNode<T> {
    pub size: T,
    pub left: T,
    pub right: T,
}

#[rustfmt::skip]
deref_derive!(
    pub struct OldStore<T>(Vec<OldNode<T>>);
    pub struct New<T>(Vec<Node<T>>);
);

impl<T: Bridge> SzbTree<T> for OldStore<T> {
    unsafe fn get_mut_left_reference(&mut self, node: T) -> *mut T {
        &mut self.get_mut(node.as_usize()).unwrap().left
    }

    unsafe fn get_mut_right_reference(&mut self, node: T) -> *mut T {
        &mut self.get_mut(node.as_usize()).unwrap().right
    }

    unsafe fn get_left_reference(&self, node: T) -> *const T {
        &self.get(node.as_usize()).unwrap().left
    }

    unsafe fn get_right_reference(&self, node: T) -> *const T {
        &self.get(node.as_usize()).unwrap().right
    }

    unsafe fn get_left(&self, node: T) -> T {
        self.get(node.as_usize()).unwrap().left
    }

    unsafe fn get_right(&self, node: T) -> T {
        self.get(node.as_usize()).unwrap().right
    }

    unsafe fn get_size(&self, node: T) -> T {
        if let Some(value) = self.get(node.as_usize()) { value.size } else { T::zero() }
    }

    unsafe fn set_left(&mut self, node: T, left: T) {
        if let Some(value) = self.get_mut(node.as_usize()) {
            value.left = left
        }
    }

    unsafe fn set_right(&mut self, node: T, right: T) {
        if let Some(value) = self.get_mut(node.as_usize()) {
            value.right = right
        }
    }

    unsafe fn set_size(&mut self, node: T, size: T) {
        if let Some(value) = self.get_mut(node.as_usize()) {
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

impl<T: Bridge> NoRecurSzbTree<T> for OldStore<T> {}

impl<T: Bridge> BTree for OldStore<T> {
    type Item = T;

    fn new(len: usize) -> Self {
        Self((0..len).map(|_| default()).collect())
    }

    fn _attach(&mut self, root: &mut Option<Self::Item>, node: Self::Item) {
        if let Some(root) = root {
            unsafe { self.attach(root, node) }
        } else {
            *root = Some(T::zero());
            unsafe { self.attach(root.as_mut().unwrap(), node) }
        }
    }

    fn _detach(&mut self, root: &mut Option<Self::Item>, node: Self::Item) {
        if let Some(root) = root {
            unsafe { self.detach(root, node) }
        }
    }

    fn is_contains(&self, root: Self::Item, node: Self::Item) -> bool {
        unsafe { <Self as SzbTree<_>>::contains(self, node, root) }
    }

    fn reset(&mut self) {
        self.0.fill(OldNode::default())
    }
}

impl<T: Bridge + LinkType> new::Tree<T> for New<T> {
    fn get(&self, idx: T) -> Option<new::Node<T>> {
        let Node { size, left, right } = self.0.get(idx.addr()).copied()?;
        Some(new::Node { size: size.addr(), left, right })
    }

    fn set(&mut self, idx: T, node: new::Node<T>) {
        let Node { size, left, right } = &mut self.0[idx.addr()];
        *size = T::from_addr(node.size);
        *left = node.left;
        *right = node.right;
    }

    fn left_mut(&mut self, idx: T) -> Option<&mut T> {
        self.0.get_mut(idx.addr())?.left.as_mut()
    }

    fn right_mut(&mut self, idx: T) -> Option<&mut T> {
        self.0.get_mut(idx.addr())?.right.as_mut()
    }

    fn is_left_of(&self, first: T, second: T) -> bool {
        first.addr() < second.addr()
    }
}

unsafe impl<T: Bridge> new::NoRecur<T> for New<T> {}

impl<T: Bridge> BTree for New<T> {
    type Item = T;

    fn new(len: usize) -> Self {
        Self((0..len).map(|_| default()).collect())
    }

    fn _attach(&mut self, root: &mut Option<Self::Item>, item: Self::Item) {
        *root = self.attach(*root, item);
    }

    fn _detach(&mut self, _root: &mut Option<Self::Item>, _item: Self::Item) {
        todo!()
    }

    fn is_contains(&self, root: Self::Item, node: Self::Item) -> bool {
        <Self as new::Tree<_>>::is_contains(self, root, node)
    }

    fn reset(&mut self) {
        self.0.fill(Node::default())
    }
}

mod dirty {
    use std::convert::{TryFrom, TryInto};

    pub fn into<T: TryFrom<usize>>(val: usize) -> T {
        unsafe { val.try_into().unwrap_unchecked() }
    }
}

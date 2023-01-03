use std::convert::{TryFrom, TryInto};
use std::default::default;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

use platform_data::LinkType;
use tap::Pipe;

use platform_trees::{new, new_v2, NoRecurSzbTree, SzbTree};

use super::BTree;

#[derive(Default)]
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

#[derive(Default)]
pub struct OldNode<T> {
    pub size: T,
    pub left: T,
    pub right: T,
}

#[rustfmt::skip]
deref_derive!(
    pub struct Store<T>(Vec<Node<T>>);
    pub struct OldStore<T>(Vec<OldNode<T>>);
    // pub struct Old<T>  (Store<T>); -- :(
    
    pub struct New<T>  (Store<T>);
    pub struct NewV2<T>(Store<T>);
);

impl<T: LinkType> Store<T> {
    pub fn new(size: usize) -> Self {
        Self((0..size).map(|_| default()).collect())
    }
}

impl<T: LinkType> OldStore<T> {
    pub fn new(size: usize) -> Self {
        Self((0..size).map(|_| default()).collect())
    }
}

impl<T: LinkType> SzbTree<T> for OldStore<T> {
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
        if let Some(value) = self.get(node.as_usize()) {
            value.size
        } else {
            T::zero()
        }
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

impl<T: LinkType> NoRecurSzbTree<T> for OldStore<T> {}

impl<T: LinkType> BTree for OldStore<T> {
    type Item = T;

    fn add(&mut self, root: &mut Option<Self::Item>, node: Self::Item) {
        if let Some(root) = root {
            unsafe { self.attach(root, node) }
        } else {
            *root = Some(T::one());
            unsafe { self.attach(root.as_mut().unwrap(), node) }
        }
    }

    fn remove(&mut self, root: &mut Option<Self::Item>, node: Self::Item) {
        if let Some(root) = root {
            unsafe { self.detach(root, node) }
        }
    }
}

impl<T: LinkType> new::Tree<T> for New<T> {
    type Item = Node<T>;

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

impl<T: LinkType> new::NoRecur<T> for New<T> {}

impl<T: LinkType> BTree for New<T> {
    type Item = T;

    fn add(&mut self, root: &mut Option<Self::Item>, item: Self::Item) {
        *root = <Self as new::NoRecur<T>>::attach(self.as_mut_slice(), *root, item);
    }

    fn remove(&mut self, root: &mut Option<Self::Item>, item: Self::Item) {
        todo!()
    }
}

mod dirty {
    use std::convert::{TryFrom, TryInto};
    use std::fmt::Debug;

    pub(crate) fn into<T: TryFrom<usize>>(val: usize) -> T
    where
        <T as TryFrom<usize>>::Error: Debug,
    {
        // val.try_into().expect("dirty hack => bug in `core`")
        unsafe { val.try_into().unwrap_unchecked() }
    }
}

impl<T: LinkType> new_v2::Tree for NewV2<T> {
    type Item = Node<T>;

    #[inline]
    fn get(item: &Self::Item) -> new_v2::Node {
        new_v2::Node {
            size: item.size.as_usize(),
            left: item.left.map(T::as_usize),
            right: item.right.map(T::as_usize),
        }
    }

    #[inline(always)]
    fn set(item: &mut Self::Item, val: new_v2::Node) {
        item.size = val.size.pipe(dirty::into);
        item.left = val.left.map(dirty::into);
        item.right = val.right.map(dirty::into);
    }

    fn is_left_of(_: &[Self::Item], first: usize, second: usize) -> bool {
        first < second
    }
}

impl<T: LinkType> new_v2::NoRecur for NewV2<T> {}

impl<T: LinkType> BTree for NewV2<T> {
    type Item = T;

    fn add(&mut self, root: &mut Option<Self::Item>, node: Self::Item) {
        *root = <Self as new_v2::NoRecur>::attach(
            self.as_mut_slice(),
            root.map(T::as_usize),
            node.as_usize(),
        )
        .map(dirty::into)
    }

    fn remove(&mut self, root: &mut Option<Self::Item>, node: Self::Item) {
        todo!()
    }
}

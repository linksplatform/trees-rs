use platform_data::LinkType;
use platform_trees::{NewNewNoRecur, NewNewTree, NewNoRecur, NewTree, NoRecurSzbTree, SzbTree};
use std::convert::TryInto;
use std::default::default;
use std::marker::PhantomData;
use tap::Pipe;

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

    pub fn as_mut_slice(&mut self) -> &mut [Node<T>] {
        self.place.as_mut_slice()
    }

    fn get(&self, index: T) -> Option<&Node<T>> {
        self.place.get(index.as_usize())
    }

    fn get_mut(&mut self, index: T) -> Option<&mut Node<T>> {
        self.place.get_mut(index.as_usize())
    }
}

macro_rules! ignore {
    ($($tt:tt)*) => {
        let _ = (|| -> Option<_> {
            $($tt)*;
            Some(())
        })();
    };
}

#[derive(Debug, Default)]
pub struct Node2<T> {
    pub size: T,
    pub left: Option<T>,
    pub right: Option<T>,
}

pub struct Dummy<T>(PhantomData<T>);

impl<T: LinkType> NewTree<T> for Dummy<T> {
    type Item = Node2<T>;

    fn size(slice: &[Self::Item], idx: T) -> Option<T> {
        let idx = idx.as_usize();
        slice.get(idx)?.size.pipe(Some)
    }

    fn left(slice: &[Self::Item], idx: T) -> Option<T> {
        let idx = idx.as_usize();
        slice.get(idx)?.left
    }

    fn right(slice: &[Self::Item], idx: T) -> Option<T> {
        let idx = idx.as_usize();
        slice.get(idx)?.right
    }

    fn set_size(slice: &mut [Self::Item], idx: T, value: T) {
        let idx = idx.as_usize();
        ignore! {
            slice.get_mut(idx)?.size = value
        }
    }

    fn set_left(slice: &mut [Self::Item], idx: T, value: Option<T>) {
        let idx = idx.as_usize();
        ignore! {
            slice.get_mut(idx)?.left = value;
        }
    }

    fn set_right(slice: &mut [Self::Item], idx: T, value: Option<T>) {
        let idx = idx.as_usize();

        ignore! {
            slice.get_mut(idx)?.right = value;
        }
    }

    fn is_left_of(_: &[Self::Item], first: T, second: T) -> bool {
        first < second
    }

    fn is_right_of(_: &[Self::Item], first: T, second: T) -> bool {
        first > second
    }
}

impl<T: LinkType> NewNoRecur<T> for Dummy<T> {}

pub struct Dummy2<T>(PhantomData<T>);

impl<T: LinkType + From<usize>> NewNewTree for Dummy2<T> {
    type Item = Node2<T>;

    fn get(item: &Self::Item) -> platform_trees::Node {
        platform_trees::Node {
            size: item.size.as_usize(),
            left: item.left.map(|x| x.as_usize()),
            right: item.right.map(|x| x.as_usize()),
        }
    }

    fn set(item: &mut Self::Item, val: platform_trees::Node) {
        item.size = val.size.try_into().unwrap();
        item.left = val.left.map(Into::into);
        item.right = val.right.map(Into::into);
    }

    fn is_left_of(_: &[Self::Item], first: usize, second: usize) -> bool {
        first < second
    }
}

impl<T: LinkType + From<usize>> NewNewNoRecur for Dummy2<T> {}

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

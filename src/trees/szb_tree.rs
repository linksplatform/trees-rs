use platform_data::LinkType;
use std::collections::HashMap;
use tap::{Pipe, Tap};

macro_rules! tri {
    ($expr:expr) => {
        match $expr {
            Some(x) => x,
            None => return false,
        }
    };
}

macro_rules! fn_set {
    ($($name:ident => $set:ident: $ty:ty)*) => {
        $(
            fn $name(slice: &mut [Self::Item], idx: usize, $set: $ty) {
                Self::_get(slice, idx).map(|node| Self::_set(slice, idx, Node { $set, ..node }));
            }
        )*
    };
}

pub struct Node {
    pub size: usize,
    pub left: Option<usize>,
    pub right: Option<usize>,
}

pub trait NewNewTree {
    type Item;

    fn get(item: &Self::Item) -> Node;
    fn set(item: &mut Self::Item, val: Node);

    fn _get(slice: &[Self::Item], idx: usize) -> Option<Node> {
        slice.get(idx).map(Self::get)
    }

    fn _set(slice: &mut [Self::Item], idx: usize, node: Node) {
        slice.get_mut(idx).map(|val| Self::set(val, node));
    }

    fn size(slice: &[Self::Item], idx: usize) -> Option<usize> {
        Self::_get(slice, idx)?.size.pipe(Some)
    }

    fn left(slice: &[Self::Item], idx: usize) -> Option<usize> {
        Self::_get(slice, idx)?.left
    }

    fn right(slice: &[Self::Item], idx: usize) -> Option<usize> {
        Self::_get(slice, idx)?.right
    }

    fn_set! {
        set_size => size: usize
        set_left => left: Option<usize>
        set_right => right: Option<usize>
    }

    fn push_left(slice: &mut [Self::Item], root: usize, idx: usize) {
        if Self::left(slice, root).is_none() {
            Self::inc_size(slice, root);
        }
        Self::set_left(slice, root, Some(idx));
    }

    fn is_left_of(slice: &[Self::Item], first: usize, second: usize) -> bool;
    fn is_right_of(slice: &[Self::Item], first: usize, second: usize) -> bool {
        first != second && !Self::is_left_of(slice, first, second)
    }

    fn left_size(slice: &[Self::Item], idx: usize) -> Option<usize> {
        Self::left(slice, idx).and_then(|idx| Self::size(slice, idx))
    }

    fn right_size(slice: &[Self::Item], idx: usize) -> Option<usize> {
        Self::right(slice, idx).and_then(|idx| Self::size(slice, idx))
    }

    fn rightest(slice: &[Self::Item], mut current: usize) -> Option<usize> {
        while let Some(next) = Self::right(slice, current) {
            current = next;
        }
        Some(current)
    }

    fn leftest(slice: &[Self::Item], mut current: usize) -> Option<usize> {
        while let Some(next) = Self::left(slice, current) {
            current = next;
        }
        Some(current)
    }

    fn next(slice: &[Self::Item], idx: usize) -> Option<usize> {
        Self::right(slice, idx).and_then(|idx| Self::leftest(slice, idx))
    }

    fn prev(slice: &[Self::Item], idx: usize) -> Option<usize> {
        Self::left(slice, idx).and_then(|idx| Self::rightest(slice, idx))
    }

    fn is_contains(slice: &[Self::Item], mut root: usize, idx: usize) -> bool {
        loop {
            // match (
            //     Self::is_left_of(slice, idx, root),
            //     Self::is_right_of(slice, idx, root),
            // ) {
            //     (true, _) => root = tri!(Self::left(slice, root)),
            //     (_, true) => root = tri!(Self::right(slice, root)),
            //     (_, _) => return false,
            // }
            println!("{root} : {idx}");
            if Self::is_left_of(slice, idx, root) {
                root = tri!(Self::left(slice, root));
            } else if Self::is_right_of(slice, idx, root) {
                root = tri!(Self::right(slice, root));
            } else {
                return true;
            }
        }
    }

    fn inc_size(slice: &mut [Self::Item], idx: usize) {
        Self::size(slice, idx).map(|size| Self::set_size(slice, idx, size + 1));
    }

    fn dec_size(slice: &mut [Self::Item], idx: usize) {
        Self::size(slice, idx).map(|size| Self::set_size(slice, idx, size - 1));
    }

    fn fix_size(slice: &mut [Self::Item], idx: usize) {
        Self::set_size(
            slice,
            idx,
            Self::left_size(slice, idx).unwrap_or_default()
                + Self::right_size(slice, idx).unwrap_or_default()
                + 1,
        )
    }

    fn clear(slice: &mut [Self::Item], idx: usize) {
        Self::set_left(slice, idx, None);
        Self::set_right(slice, idx, None);
        Self::set_size(slice, idx, 0);
    }

    fn rotate_left(slice: &mut [Self::Item], root: usize) -> Option<usize> {
        let right = Self::right(slice, root)?;
        Self::left(slice, right).map(|left| Self::set_right(slice, root, Some(left)));
        Self::set_left(slice, right, Some(root));
        Self::set_size(slice, right, Self::size(slice, root)?);
        Self::fix_size(slice, root);
        Some(right)
    }

    fn rotate_right(slice: &mut [Self::Item], root: usize) -> Option<usize> {
        let left = Self::left(slice, root)?;
        Self::right(slice, left).map(|right| Self::set_left(slice, root, Some(right)));
        Self::set_right(slice, left, Some(root));
        Self::set_size(slice, left, Self::size(slice, root)?);
        Self::fix_size(slice, root);
        Some(left)
    }
}

pub trait NewTree<T: LinkType> {
    type Item;

    fn size(slice: &[Self::Item], idx: T) -> Option<T>;
    fn left(slice: &[Self::Item], idx: T) -> Option<T>;
    fn right(slice: &[Self::Item], idx: T) -> Option<T>;

    fn set_size(slice: &mut [Self::Item], idx: T, value: T);
    fn set_left(slice: &mut [Self::Item], idx: T, value: Option<T>);
    fn set_right(slice: &mut [Self::Item], idx: T, value: Option<T>);

    fn is_left_of(slice: &[Self::Item], first: T, second: T) -> bool;
    fn is_right_of(slice: &[Self::Item], first: T, second: T) -> bool {
        first != second && !Self::is_left_of(slice, first, second)
    }

    fn left_size(slice: &[Self::Item], idx: T) -> Option<T> {
        Self::left(slice, idx).and_then(|idx| Self::size(slice, idx))
    }

    fn right_size(slice: &[Self::Item], idx: T) -> Option<T> {
        Self::right(slice, idx).and_then(|idx| Self::size(slice, idx))
    }

    fn rightest(slice: &[Self::Item], mut current: T) -> Option<T> {
        while let Some(next) = Self::right(slice, current) {
            current = next;
        }
        Some(current)
    }

    fn leftest(slice: &[Self::Item], mut current: T) -> Option<T> {
        while let Some(next) = Self::left(slice, current) {
            current = next;
        }
        Some(current)
    }

    fn next(slice: &[Self::Item], idx: T) -> Option<T> {
        Self::right(slice, idx).and_then(|idx| Self::leftest(slice, idx))
    }

    fn prev(slice: &[Self::Item], idx: T) -> Option<T> {
        Self::left(slice, idx).and_then(|idx| Self::rightest(slice, idx))
    }

    fn is_contains(slice: &[Self::Item], mut root: T, idx: T) -> bool {
        loop {
            // match (
            //     Self::is_left_of(slice, idx, root),
            //     Self::is_right_of(slice, idx, root),
            // ) {
            //     (true, _) => root = tri!(Self::left(slice, root)),
            //     (_, true) => root = tri!(Self::right(slice, root)),
            //     (_, _) => return false,
            // }
            println!("{root} : {idx}");
            if Self::is_left_of(slice, idx, root) {
                root = tri!(Self::left(slice, root));
            } else if Self::is_right_of(slice, idx, root) {
                root = tri!(Self::right(slice, root));
            } else {
                return true;
            }
        }
    }

    #[must_use]
    fn inc_size(slice: &mut [Self::Item], idx: T) -> Option<()> {
        Self::set_size(slice, idx, Self::size(slice, idx)? + T::one()).pipe(Some)
    }

    #[must_use]
    fn dec_size(slice: &mut [Self::Item], idx: T) -> Option<()> {
        Self::set_size(slice, idx, Self::size(slice, idx)? - T::one()).pipe(Some)
    }

    #[must_use]
    fn fix_size(slice: &mut [Self::Item], idx: T) -> Option<()> {
        Self::set_size(
            slice,
            idx,
            Self::left_size(slice, idx)? + Self::right_size(slice, idx)? + T::one(),
        )
        .pipe(Some)
    }

    fn clear(slice: &mut [Self::Item], idx: T) {
        Self::set_left(slice, idx, None);
        Self::set_right(slice, idx, None);
        Self::set_size(slice, idx, T::zero());
    }

    #[must_use]
    fn rotate_left(slice: &mut [Self::Item], root: T) -> Option<T> {
        let right = Self::right(slice, root)?;
        Self::left(slice, root)?.pipe(|right| Self::set_right(slice, root, Some(right)));
        Self::set_left(slice, right, Some(root));
        Self::set_size(slice, right, Self::size(slice, root)?);
        Self::fix_size(slice, root)?;
        Some(right)
    }

    #[must_use]
    fn rotate_right(slice: &mut [Self::Item], root: T) -> Option<T> {
        let left = Self::left(slice, root)?;
        Self::right(slice, root)?.pipe(|right| Self::set_left(slice, root, Some(right)));
        Self::set_right(slice, left, Some(root));
        Self::set_size(slice, left, Self::size(slice, root)?);
        Self::fix_size(slice, root)?;
        Some(left)
    }
}

pub trait SzbTree<T: LinkType> {
    unsafe fn get_mut_left_reference(&mut self, node: T) -> *mut T;

    unsafe fn get_mut_right_reference(&mut self, node: T) -> *mut T;

    unsafe fn get_left_reference(&self, node: T) -> *const T;

    unsafe fn get_right_reference(&self, node: T) -> *const T;

    unsafe fn get_left(&self, node: T) -> T;

    unsafe fn get_right(&self, node: T) -> T;

    unsafe fn get_size(&self, node: T) -> T;

    unsafe fn set_left(&mut self, node: T, left: T);

    unsafe fn set_right(&mut self, node: T, right: T);

    unsafe fn set_size(&mut self, node: T, size: T);

    unsafe fn first_is_to_the_left_of_second(&self, first: T, second: T) -> bool;

    unsafe fn first_is_to_the_right_of_second(&self, first: T, second: T) -> bool;

    unsafe fn get_left_or_default(&self, node: T) -> T {
        if node == T::zero() {
            T::zero()
        } else {
            self.get_left(node)
        }
    }

    unsafe fn get_right_or_default(&self, node: T) -> T {
        if node == T::zero() {
            T::zero()
        } else {
            self.get_right(node)
        }
    }

    unsafe fn get_size_or_zero(&self, node: T) -> T {
        if node == T::zero() {
            T::zero()
        } else {
            self.get_size(node)
        }
    }

    unsafe fn inc_size(&mut self, node: T) {
        self.set_size(node, self.get_size(node) + T::one());
    }

    unsafe fn dec_size(&mut self, node: T) {
        self.set_size(node, self.get_size(node) - T::one());
    }

    unsafe fn get_left_size(&self, node: T) -> T {
        self.get_size_or_zero(self.get_left_or_default(node))
    }

    unsafe fn get_right_size(&self, node: T) -> T {
        self.get_size_or_zero(self.get_right_or_default(node))
    }

    unsafe fn fix_size(&mut self, node: T) {
        self.set_size(
            node,
            (self.get_left_size(node) + self.get_right_size(node)) + T::one(),
        );
    }

    unsafe fn left_rotate(&mut self, root: *mut T) {
        *root = self.left_rotate_core(*root);
    }

    unsafe fn left_rotate_core(&mut self, root: T) -> T {
        let right = self.get_right(root);
        self.set_right(root, self.get_left(right));
        self.set_left(right, root);
        self.set_size(right, self.get_size(root));
        self.fix_size(root);
        right
    }

    unsafe fn right_rotate(&mut self, root: *mut T) {
        *root = self.right_rotate_core(*root);
    }

    unsafe fn right_rotate_core(&mut self, root: T) -> T {
        let left = self.get_left(root);
        self.set_left(root, self.get_right(left));
        self.set_right(left, root);
        self.set_size(left, self.get_size(root));
        self.fix_size(root);
        left
    }

    unsafe fn get_rightest(&self, mut current: T) -> T {
        let mut current_right = self.get_right(current);
        while current_right != T::zero() {
            current = current_right;
            current_right = self.get_right(current);
        }
        current
    }

    unsafe fn get_leftest(&self, mut current: T) -> T {
        let mut current_left = self.get_left(current);
        while current_left != T::zero() {
            current = current_left;
            current_left = self.get_left(current);
        }
        current
    }

    unsafe fn get_next(&self, node: T) -> T {
        self.get_leftest(self.get_right(node))
    }

    unsafe fn get_previous(&self, node: T) -> T {
        self.get_rightest(self.get_left(node))
    }

    unsafe fn contains(&self, node: T, mut root: T) -> bool {
        while root != T::zero() {
            if self.first_is_to_the_left_of_second(node, root) {
                root = self.get_left(root);
            } else if self.first_is_to_the_right_of_second(node, root) {
                root = self.get_right(root);
            } else {
                return true;
            }
        }
        false
    }

    unsafe fn clear_node(&mut self, node: T) {
        self.set_left(node, T::zero());
        self.set_right(node, T::zero());
        self.set_size(node, T::zero());
    }
}

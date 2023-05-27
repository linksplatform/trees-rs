use {
    platform_data::LinkType,
    std::{ptr, ptr::NonNull},
};

#[derive(Debug)]
pub struct Node<T: LinkType> {
    pub size: usize,
    pub left: Option<T>,
    pub right: Option<T>,
}

macro_rules! fn_set {
    ($($name:ident => $set:ident: $ty:ty)*) => {$(
        fn $name(&mut self, idx: T, $set: $ty) {
            if let Some(node) = self.get(idx) {
                self.set(idx, Node { $set, ..node });
            }
        }
    )*};
}

pub trait Tree<T: LinkType> {
    fn get(&self, idx: T) -> Option<Node<T>>;
    fn set(&mut self, idx: T, node: Node<T>);

    fn left_mut(&mut self, idx: T) -> Option<&mut T>;
    fn right_mut(&mut self, idx: T) -> Option<&mut T>;

    fn is_left_of(&self, first: T, second: T) -> bool;

    fn is_right_of(&self, first: T, second: T) -> bool {
        first.addr() != second.addr() && !self.is_left_of(first, second)
    }

    fn size(&self, idx: T) -> Option<usize> {
        try { self.get(idx).unwrap().size }
    }

    fn left(&self, idx: T) -> Option<T> {
        self.get(idx).unwrap().left
    }

    fn right(&self, idx: T) -> Option<T> {
        self.get(idx).unwrap().right
    }

    fn_set! {
        set_size => size: usize
        set_left => left: Option<T>
        set_right => right: Option<T>
    }

    fn left_size(&self, idx: T) -> Option<usize> {
        self.left(idx).and_then(|idx| self.size(idx))
    }

    fn right_size(&self, idx: T) -> Option<usize> {
        self.right(idx).and_then(|idx| self.size(idx))
    }

    fn rightest(&self, mut current: T) -> T {
        while let Some(next) = self.right(current) {
            current = next;
        }
        current
    }

    fn leftest(&self, mut current: T) -> T {
        while let Some(next) = self.left(current) {
            current = next;
        }
        current
    }

    fn next(&self, idx: T) -> Option<T> {
        self.right(idx).map(|idx| self.leftest(idx))
    }

    fn prev(&self, idx: T) -> Option<T> {
        self.left(idx).map(|idx| self.rightest(idx))
    }

    fn is_contains(&self, mut root: T, idx: T) -> bool {
        loop {
            //println!("search: {}", root.addr());
            if self.is_left_of(idx, root) {
                root = tri! { self.left(root) };
            } else if self.is_right_of(idx, root) {
                root = tri! { self.right(root) };
            } else {
                break true;
            }
        }
    }

    fn inc_size(&mut self, idx: T) {
        if let Some(size) = self.size(idx) {
            self.set_size(idx, size + 1)
        }
    }

    fn dec_size(&mut self, idx: T) {
        if let Some(size) = self.size(idx) {
            self.set_size(idx, size - 1)
        }
    }

    fn fix_size(&mut self, idx: T) {
        self.set_size(
            idx,
            self.left_size(idx).unwrap_or_default() + self.right_size(idx).unwrap_or_default() + 1,
        )
    }

    fn clear(&mut self, idx: T) {
        self.set(idx, Node { size: 0, left: None, right: None })
    }

    #[must_use]
    fn rotate_left(&mut self, root: T) -> Option<T> {
        let right = self.right(root).unwrap();
        self.set_right(root, self.left(right));
        self.set_left(right, Some(root));
        self.set_size(right, self.size(root).unwrap());
        self.fix_size(root);
        Some(right)
    }

    #[must_use]
    fn rotate_right(&mut self, root: T) -> Option<T> {
        let left = self.left(root).unwrap();
        self.set_left(root, self.right(left));
        self.set_right(left, Some(root));
        self.set_size(left, self.size(root).unwrap());
        self.fix_size(root);
        Some(left)
    }
}

pub unsafe trait NoRecur<T: LinkType>: Tree<T> {
    fn attach(&mut self, root: Option<T>, idx: T) -> Option<T> {
        if let Some(mut root) = root {
            unsafe { attach_impl(self, &mut root, idx).unwrap() };
            Some(root)
        } else {
            self.set_size(idx, 1);
            Some(idx)
        }
    }
}

unsafe fn attach_impl<T: LinkType, Tree>(tree: &mut Tree, mut root: *mut T, idx: T) -> Option<()>
where
    Tree: NoRecur<T> + ?Sized,
{
    loop {
        if tree.is_left_of(idx, *root) {
            let Some(left) = tree.left_mut(*root) else {
                tree.inc_size(*root);
                tree.set_size(idx, 1);
                tree.set_left(*root, Some(idx));
                return Some(());
            };
            let left = left as *mut T;

            let left_size = tree.size(*left).unwrap();
            let right_size = tree.right_size(*root).unwrap_or_default();

            if tree.is_left_of(idx, *left) {
                if left_size >= right_size {
                    *root = tree.rotate_right(*root).unwrap();
                } else {
                    tree.inc_size(*root);
                    root = left;
                }
            } else {
                let lr_size = tree.right_size(*left).unwrap_or_default();
                if lr_size >= right_size {
                    if lr_size == 0 && right_size == 0 {
                        tree.set_left(idx, Some(*left));
                        tree.set_right(idx, Some(*root));
                        tree.set_size(idx, left_size + 2);
                        tree.set_left(*root, None);
                        tree.set_size(*root, 1);
                        *root = idx;
                        return Some(());
                    }
                    *left = tree.rotate_left(*left).unwrap();
                    *root = tree.rotate_right(*root).unwrap();
                } else {
                    tree.inc_size(*root);
                    root = left;
                }
            }
        } else {
            let Some(right) = tree.right_mut(*root) else {
                tree.inc_size(*root);
                tree.set_size(idx, 1);
                tree.set_right(*root, Some(idx));
                return Some(());
            };
            let right = right as *mut T;

            let right_size = tree.size(*right).unwrap();
            let left_size = tree.left_size(*root).unwrap_or_default();

            if tree.is_right_of(idx, *right) {
                if right_size >= left_size {
                    *root = tree.rotate_left(*root).unwrap();
                } else {
                    tree.inc_size(*root);
                    root = right;
                }
            } else {
                let rl_size = tree.left_size(*right).unwrap_or_default();
                if rl_size >= left_size {
                    if rl_size == 0 && left_size == 0 {
                        tree.set_left(idx, Some(*root));
                        tree.set_right(idx, Some(*right));
                        tree.set_size(idx, right_size + 2);
                        tree.set_right(*root, None);
                        tree.set_size(*root, 1);
                        *root = idx;
                        return Some(());
                    }
                    *right = tree.rotate_right(*right).unwrap();
                    *root = tree.rotate_left(*root).unwrap();
                } else {
                    tree.inc_size(*root);
                    root = right;
                }
            }
        }
    }
}

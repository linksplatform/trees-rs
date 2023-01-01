use platform_data::LinkType;

use crate::trees::NewNewTree;
use crate::{NewTree, SzbTree};

fn attach_impl2<Tree>(slice: &mut [Tree::Item], mut root: usize, idx: usize) -> Option<usize>
where
    Tree: NewNewTree,
{
    loop {
        let right = Tree::right(slice, root);
        if Tree::is_left_of(slice, idx, root) {
            let Some(left) = Tree::left(slice, root) else {
                Tree::inc_size(slice, root);
                Tree::set_size(slice, idx, 1);
                Tree::set_left(slice, root, Some(idx));
                return Some(root);
            };
            
            let left_size = Tree::size(slice, left)?;
            let right_size = Tree::right_size(slice, root).unwrap_or_default();
            if Tree::is_left_of(slice, idx, left) {
                if left_size >= right_size {
                    root = Tree::rotate_right(slice, root)?;
                } else {
                    Tree::inc_size(slice, root);
                    root = left;
                }
            } else {
                let lr_size =
                    Tree::right(slice, left).and_then(|right| Tree::size(slice, right))?;
                if lr_size >= right_size {
                    if lr_size == 0 && right_size == 0 {
                        Tree::set_left(slice, idx, Some(left));
                        Tree::set_right(slice, idx, Some(root));
                        Tree::set_size(slice, idx, left_size + 2);
                        Tree::set_left(slice, root, None);
                        Tree::set_size(slice, root, 1);
                        return Some(root);
                    } else {
                        let new = Tree::rotate_left(slice, left)?;
                        Tree::set_left(slice, root, Some(new));
                        root = Tree::rotate_right(slice, root)?;
                    }
                } else {
                    Tree::inc_size(slice, root);
                    root = left;
                }
            }
        } else {
            match right {
                None => {
                    Tree::inc_size(slice, root);
                    Tree::set_size(slice, idx, 1);
                    Tree::set_right(slice, root, Some(idx));
                    return Some(root);
                }
                Some(right) => {
                    let right_size = Tree::size(slice, right)?;
                    let left_size = Tree::left_size(slice, root).unwrap_or_default();
                    if Tree::is_right_of(slice, idx, right) {
                        if right_size >= left_size {
                            root = Tree::rotate_left(slice, root)?;
                        } else {
                            Tree::inc_size(slice, root);
                            root = right;
                        }
                    } else {
                        let rl_size =
                            Tree::left(slice, right).and_then(|left| Tree::size(slice, left))?;
                        if rl_size >= left_size {
                            if rl_size == 0 && left_size == 0 {
                                Tree::set_left(slice, idx, Some(root));
                                Tree::set_right(slice, idx, Some(right));
                                Tree::set_size(slice, idx, right_size + 2);
                                Tree::set_left(slice, root, None);
                                Tree::set_size(slice, root, 1);
                                return Some(root);
                            } else {
                                let new = Tree::rotate_right(slice, right)?;
                                Tree::set_right(slice, root, Some(new));
                                root = Tree::rotate_left(slice, root)?;
                            }
                        } else {
                            Tree::inc_size(slice, root);
                            root = right;
                        }
                    }
                }
            }
        }
    }
}

fn attach_impl<T, Tree>(slice: &mut [Tree::Item], mut root: T, idx: T) -> Option<T>
where
    T: LinkType,
    Tree: NewTree<T>,
{
    loop {
        let left = Tree::left(slice, root);
        let right = Tree::right(slice, root);
        if Tree::is_left_of(slice, idx, root) {
            match left {
                None => {
                    Tree::inc_size(slice, root)?;
                    Tree::set_size(slice, idx, T::one());
                    Tree::set_left(slice, root, Some(idx));
                    return Some(root);
                }
                Some(left) => {
                    let left_size = Tree::size(slice, left)?;
                    let right_size = Tree::right_size(slice, root).unwrap_or_default();
                    if Tree::is_left_of(slice, idx, left) {
                        if left_size >= right_size {
                            root = Tree::rotate_right(slice, root)?;
                        } else {
                            Tree::inc_size(slice, root)?;
                            root = left;
                        }
                    } else {
                        let lr_size =
                            Tree::right(slice, left).and_then(|right| Tree::size(slice, right))?;
                        if lr_size >= right_size {
                            if lr_size == T::zero() && right_size == T::zero() {
                                Tree::set_left(slice, idx, Some(left));
                                Tree::set_right(slice, idx, Some(root));
                                Tree::set_size(slice, idx, left_size + T::two());
                                Tree::set_left(slice, root, None);
                                Tree::set_size(slice, root, T::one());
                                return Some(root);
                            } else {
                                let new = Tree::rotate_left(slice, left)?;
                                Tree::set_left(slice, root, Some(new));
                                root = Tree::rotate_right(slice, root)?;
                            }
                        } else {
                            Tree::inc_size(slice, root)?;
                            root = left;
                        }
                    }
                }
            }
        } else {
            match right {
                None => {
                    Tree::inc_size(slice, root)?;
                    Tree::set_size(slice, idx, T::one());
                    Tree::set_right(slice, root, Some(idx));
                    return Some(root);
                }
                Some(right) => {
                    let right_size = Tree::size(slice, right)?;
                    let left_size = Tree::left_size(slice, root).unwrap_or_default();
                    if Tree::is_right_of(slice, idx, right) {
                        if right_size >= left_size {
                            root = Tree::rotate_left(slice, root)?;
                        } else {
                            Tree::inc_size(slice, root)?;
                            root = right;
                        }
                    } else {
                        let rl_size =
                            Tree::left(slice, right).and_then(|left| Tree::size(slice, left))?;
                        if rl_size >= left_size {
                            if rl_size == T::zero() && left_size == T::zero() {
                                Tree::set_left(slice, idx, Some(root));
                                Tree::set_right(slice, idx, Some(right));
                                Tree::set_size(slice, idx, right_size + T::two());
                                Tree::set_left(slice, root, None);
                                Tree::set_size(slice, root, T::one());
                                return Some(root);
                            } else {
                                let new = Tree::rotate_right(slice, right)?;
                                Tree::set_right(slice, root, Some(new));
                                root = Tree::rotate_left(slice, root)?;
                            }
                        } else {
                            Tree::inc_size(slice, root)?;
                            root = right;
                        }
                    }
                }
            }
        }
    }
}

pub trait NewNewNoRecur: NewNewTree + Sized {
    fn attach(slice: &mut [Self::Item], root: Option<usize>, idx: usize) -> Option<usize> {
        if let Some(root) = root {
            attach_impl2::<Self>(slice, root, idx)
        } else {
            Self::set_size(slice, idx, 1);
            Some(idx)
        }
    }
}

pub trait NewNoRecur<T: LinkType>: NewTree<T> + Sized {
    fn attach(slice: &mut [Self::Item], root: Option<T>, idx: T) -> Option<T> {
        if let Some(root) = root {
            attach_impl::<_, Self>(slice, root, idx)
        } else {
            Self::set_size(slice, idx, T::one());
            Some(idx)
        }
    }

    fn detach(slice: &mut [Self::Item], root: Option<T>, idx: T) -> Option<T> {
        todo!()
    }
}

pub trait NoRecurSzbTree<T: LinkType>: SzbTree<T> {
    unsafe fn attach(&mut self, root: *mut T, node: T) {
        if *root == T::zero() {
            self.set_size(node, T::one());
            *root = node;
            return;
        }
        self.attach_core(root, node);
    }
    unsafe fn detach(&mut self, root: *mut T, node: T) {
        self.detach_core(root, node);
    }

    unsafe fn attach_core(&mut self, mut root: *mut T, node: T) {
        loop {
            let left = self.get_mut_left_reference(*root);
            let left_size = self.get_size_or_zero(*left);
            let right = self.get_mut_right_reference(*root);
            let right_size = self.get_size_or_zero(*right);
            if self.first_is_to_the_left_of_second(node, *root) {
                if *left == T::zero() {
                    self.inc_size(*root);
                    self.set_size(node, T::one());
                    *left = node;
                    return;
                }
                if self.first_is_to_the_left_of_second(node, *left) {
                    if (left_size + T::one()) > right_size {
                        self.right_rotate(root);
                    } else {
                        self.inc_size(*root);
                        root = left;
                    }
                } else {
                    let left_right_size = self.get_size_or_zero(self.get_right(*left));
                    if (left_right_size + T::one()) > right_size {
                        if left_right_size == T::zero() && right_size == T::zero() {
                            self.set_left(node, *left);
                            self.set_right(node, *root);
                            self.set_size(node, left_size + T::one() + T::one());
                            self.set_left(*root, T::zero());
                            self.set_size(*root, T::one());
                            *root = node;
                            return;
                        }
                        self.left_rotate(left);
                        self.right_rotate(root);
                    } else {
                        self.inc_size(*root);
                        root = left;
                    }
                }
            } else {
                if *right == T::zero() {
                    self.inc_size(*root);
                    self.set_size(node, T::one());
                    *right = node;
                    return;
                }
                if self.first_is_to_the_right_of_second(node, *right) {
                    if (right_size + T::one()) > left_size {
                        self.left_rotate(root);
                    } else {
                        self.inc_size(*root);
                        root = right;
                    }
                } else {
                    let right_left_size = self.get_size_or_zero(self.get_left(*right));
                    if (right_left_size + T::one()) > left_size {
                        if right_left_size == T::zero() && left_size == T::zero() {
                            self.set_left(node, *root);
                            self.set_right(node, *right);
                            self.set_size(node, right_size + T::one() + T::one());
                            self.set_right(*root, T::zero());
                            self.set_size(*root, T::one());
                            *root = node;
                            return;
                        }
                        self.right_rotate(right);
                        self.left_rotate(root);
                    } else {
                        self.inc_size(*root);
                        root = right;
                    }
                }
            }
        }
    }

    unsafe fn detach_core(&mut self, mut root: *mut T, node: T) {
        loop {
            let left = self.get_mut_left_reference(*root);
            let left_size = self.get_size_or_zero(*left);
            let right = self.get_mut_right_reference(*root);
            let right_size = self.get_size_or_zero(*right);
            if self.first_is_to_the_left_of_second(node, *root) {
                let decremented_left_size = left_size - T::one();
                if self.get_size_or_zero(self.get_right_or_default(*right)) > decremented_left_size
                {
                    self.left_rotate(root);
                } else if self.get_size_or_zero(self.get_left_or_default(*right))
                    > decremented_left_size
                {
                    self.right_rotate(right);
                    self.left_rotate(root);
                } else {
                    self.dec_size(*root);
                    root = left;
                }
            } else if self.first_is_to_the_right_of_second(node, *root) {
                let decremented_right_size = right_size - T::one();
                if self.get_size_or_zero(self.get_left_or_default(*left)) > decremented_right_size {
                    self.right_rotate(root);
                } else if self.get_size_or_zero(self.get_right_or_default(*left))
                    > decremented_right_size
                {
                    self.left_rotate(left);
                    self.right_rotate(root);
                } else {
                    self.dec_size(*root);
                    root = right;
                }
            } else {
                if left_size > T::zero() && right_size > T::zero() {
                    let replacement;
                    if left_size > right_size {
                        replacement = self.get_rightest(*left);
                        self.detach_core(left, replacement);
                    } else {
                        replacement = self.get_leftest(*right);
                        self.detach_core(right, replacement);
                    }
                    self.set_left(replacement, *left);
                    self.set_right(replacement, *right);
                    self.set_size(replacement, left_size + right_size);
                    *root = replacement;
                } else if left_size > T::zero() {
                    *root = *left;
                } else if right_size > T::zero() {
                    *root = *right;
                } else {
                    *root = T::zero();
                }
                self.clear_node(node);
                return;
            }
        }
    }
}

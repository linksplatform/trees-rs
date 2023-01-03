use crate::SzbTree;
use platform_data::LinkType;

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

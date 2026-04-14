use crate::LinkType;

/// Base trait for a size-balanced binary search tree (SBT).
///
/// Provides node accessors, tree rotations (`left_rotate`, `right_rotate`),
/// navigation (`get_leftest`, `get_rightest`, `get_next`, `get_previous`),
/// and queries (`contains`).
///
/// Implementors supply the storage-level operations (the first 12 methods);
/// all other methods have default implementations built on those primitives.
///
/// For iterative `attach`/`detach`, see
/// [`IterativeSizeBalancedTree`](super::IterativeSizeBalancedTree).
///
/// # Safety
///
/// All methods are `unsafe` because they operate on raw node indices
/// without bounds checking — the caller must ensure that every index
/// passed to these methods refers to a valid, allocated node.
pub trait RecursiveSizeBalancedTree<T: LinkType> {
    /// Returns a mutable raw pointer to the left-child field of `node`.
    unsafe fn get_mut_left_reference(&mut self, node: T) -> *mut T;

    /// Returns a mutable raw pointer to the right-child field of `node`.
    unsafe fn get_mut_right_reference(&mut self, node: T) -> *mut T;

    /// Returns a const raw pointer to the left-child field of `node`.
    unsafe fn get_left_reference(&self, node: T) -> *const T;

    /// Returns a const raw pointer to the right-child field of `node`.
    unsafe fn get_right_reference(&self, node: T) -> *const T;

    /// Returns the left child of `node`.
    unsafe fn get_left(&self, node: T) -> T;

    /// Returns the right child of `node`.
    unsafe fn get_right(&self, node: T) -> T;

    /// Returns the subtree size stored at `node`.
    unsafe fn get_size(&self, node: T) -> T;

    /// Sets the left child of `node`.
    unsafe fn set_left(&mut self, node: T, left: T);

    /// Sets the right child of `node`.
    unsafe fn set_right(&mut self, node: T, right: T);

    /// Sets the subtree size of `node`.
    unsafe fn set_size(&mut self, node: T, size: T);

    /// Returns `true` if `first` should be placed to the left of `second`.
    unsafe fn first_is_to_the_left_of_second(&self, first: T, second: T) -> bool;

    /// Returns `true` if `first` should be placed to the right of `second`.
    unsafe fn first_is_to_the_right_of_second(&self, first: T, second: T) -> bool;

    /// Returns the left child of `node`, or `T::from_byte(0)` if `node` is zero.
    unsafe fn get_left_or_default(&self, node: T) -> T {
        unsafe {
            if node == T::from_byte(0) {
                T::from_byte(0)
            } else {
                self.get_left(node)
            }
        }
    }

    /// Returns the right child of `node`, or `T::from_byte(0)` if `node` is zero.
    unsafe fn get_right_or_default(&self, node: T) -> T {
        unsafe {
            if node == T::from_byte(0) {
                T::from_byte(0)
            } else {
                self.get_right(node)
            }
        }
    }

    /// Returns the size of `node`, or zero if `node` is zero.
    unsafe fn get_size_or_zero(&self, node: T) -> T {
        unsafe {
            if node == T::from_byte(0) {
                T::from_byte(0)
            } else {
                self.get_size(node)
            }
        }
    }

    /// Increments the subtree size of `node` by one.
    unsafe fn inc_size(&mut self, node: T) {
        unsafe {
            self.set_size(node, self.get_size(node) + T::from_byte(1));
        }
    }

    /// Decrements the subtree size of `node` by one.
    unsafe fn dec_size(&mut self, node: T) {
        unsafe {
            self.set_size(node, self.get_size(node) - T::from_byte(1));
        }
    }

    /// Returns the size of the left subtree of `node`.
    unsafe fn get_left_size(&self, node: T) -> T {
        unsafe { self.get_size_or_zero(self.get_left_or_default(node)) }
    }

    /// Returns the size of the right subtree of `node`.
    unsafe fn get_right_size(&self, node: T) -> T {
        unsafe { self.get_size_or_zero(self.get_right_or_default(node)) }
    }

    /// Recalculates the size of `node` from its children:
    /// `size = left_size + right_size + 1`.
    unsafe fn fix_size(&mut self, node: T) {
        unsafe {
            self.set_size(
                node,
                (self.get_left_size(node) + self.get_right_size(node)) + T::from_byte(1),
            );
        }
    }

    /// Performs a left rotation at the subtree rooted at `*root`,
    /// updating `*root` in place.
    unsafe fn left_rotate(&mut self, root: *mut T) {
        unsafe {
            *root = self.left_rotate_core(*root);
        }
    }

    /// Performs a left rotation and returns the new root.
    unsafe fn left_rotate_core(&mut self, root: T) -> T {
        unsafe {
            let right = self.get_right(root);
            self.set_right(root, self.get_left(right));
            self.set_left(right, root);
            self.set_size(right, self.get_size(root));
            self.fix_size(root);
            right
        }
    }

    /// Performs a right rotation at the subtree rooted at `*root`,
    /// updating `*root` in place.
    unsafe fn right_rotate(&mut self, root: *mut T) {
        unsafe {
            *root = self.right_rotate_core(*root);
        }
    }

    /// Performs a right rotation and returns the new root.
    unsafe fn right_rotate_core(&mut self, root: T) -> T {
        unsafe {
            let left = self.get_left(root);
            self.set_left(root, self.get_right(left));
            self.set_right(left, root);
            self.set_size(left, self.get_size(root));
            self.fix_size(root);
            left
        }
    }

    /// Returns the rightmost (maximum) node in the subtree rooted at `current`.
    unsafe fn get_rightest(&self, mut current: T) -> T {
        unsafe {
            let mut current_right = self.get_right(current);
            while current_right != T::from_byte(0) {
                current = current_right;
                current_right = self.get_right(current);
            }
            current
        }
    }

    /// Returns the leftmost (minimum) node in the subtree rooted at `current`.
    unsafe fn get_leftest(&self, mut current: T) -> T {
        unsafe {
            let mut current_left = self.get_left(current);
            while current_left != T::from_byte(0) {
                current = current_left;
                current_left = self.get_left(current);
            }
            current
        }
    }

    /// Returns the in-order successor of `node`.
    unsafe fn get_next(&self, node: T) -> T {
        unsafe { self.get_leftest(self.get_right(node)) }
    }

    /// Returns the in-order predecessor of `node`.
    unsafe fn get_previous(&self, node: T) -> T {
        unsafe { self.get_rightest(self.get_left(node)) }
    }

    /// Returns `true` if `node` exists in the tree rooted at `root`.
    unsafe fn contains(&self, node: T, mut root: T) -> bool {
        unsafe {
            while root != T::from_byte(0) {
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
    }

    /// Resets `node`'s left, right, and size fields to zero.
    unsafe fn clear_node(&mut self, node: T) {
        unsafe {
            self.set_left(node, T::from_byte(0));
            self.set_right(node, T::from_byte(0));
            self.set_size(node, T::from_byte(0));
        }
    }
}

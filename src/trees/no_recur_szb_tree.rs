use platform_data::LinkType;
use crate::SzbTree;

pub trait NoRecurSzbTree<T: LinkType>: SzbTree<T> {
    fn attach(&mut self, root: &mut Option<T>, node: T) {
        if root.is_none() {
            self.set_size(node, T::funty(1));
            *root = Some(node);
            return;
        }
        self.attach_core(root, node);
    }
    fn detach(&mut self, root: &mut Option<T>, node: T) {
        self.detach_core(root, node);
    }

    fn attach_core(&mut self, root: &mut Option<T>, node: T) {
        loop {
            let left = self.get_mut_left_reference(root.as_ref().unwrap());
            let left_size = self.get_size_or_zero(left.as_ref());
            let right = self.get_mut_right_reference(root.as_ref().unwrap());
            let right_size = self.get_size_or_zero(right.as_ref());
            if self.first_is_to_the_left_of_second(node, root.as_ref().unwrap()) {
                if left.is_none() {
                    self.inc_size(root.as_mut().unwrap());
                    self.set_size(node, T::funty(1));
                    *left = Some(node);
                    return;
                }
                if self.first_is_to_the_left_of_second(node, left.as_ref().unwrap()) {
                    if (left_size + T::funty(1)) > right_size {
                        self.right_rotate(root);
                    } else {
                        self.inc_size(root.as_mut().unwrap());
                        root = left;
                    }
                } else {
                    let left_right_size = self.get_size_or_zero(self.get_right(left.as_ref().unwrap()));
                    if (left_right_size + T::funty(1)) > right_size {
                        if left_right_size == T::funty(0) && right_size == T::funty(0) {
                            self.set_left(node, left.take().unwrap());
                            self.set_right(node, root.take().unwrap());
                            self.set_size(node, left_size + T::funty(1) + T::funty(1));
                            self.set_left(root.as_mut().unwrap(), T::funty(0));
                            self.set_size(root.as_mut().unwrap(), T::funty(1));
                            *root = Some(node);
                            return;
                        }
                        self.left_rotate(left);
                        self.right_rotate(root);
                    } else {
                        self.inc_size(root.as_mut().unwrap());
                        root = left;
                    }
                }
            } else {
                if right.is_none() {
                    self.inc_size(root.as_mut().unwrap());
                    self.set_size(node, T::funty(1));
                    *right = Some(node);
                    return;
                }
                if self.first_is_to_the_right_of_second(node, right.as_ref().unwrap()) {
                    if (right_size + T::funty(1)) > left_size {
                        self.left_rotate(root);
                    } else {
                        self.inc_size(root.as_mut().unwrap());
                        root = right;
                    }
                } else {
                    let right_left_size = self.get_size_or_zero(self.get_left(right.as_ref().unwrap()));
                    if (right_left_size + T::funty(1)) > left_size {
                        if right_left_size == T::funty(0) && left_size == T::funty(0) {
                            self.set_left(node, root.take().unwrap());
                            self.set_right(node, right.take().unwrap());
                            self.set_size(node, right_size +
T::funty(1) + T::funty(1));
self.set_right(root.as_mut().unwrap(), T::funty(0));
self.set_size(root.as_mut().unwrap(), T::funty(1));
*root = Some(node);
return;
}
self.right_rotate(right);
self.left_rotate(root);
} else {
self.inc_size(root.as_mut().unwrap());
root = right;
}
}
}
}
}
fn detach_core(&mut self, root: &mut Option<T>, node: T) {
    loop {
        let left = self.get_mut_left_reference(root.as_ref().unwrap());
        let left_size = self.get_size_or_zero(left.as_ref());
        let right = self.get_mut_right_reference(root.as_ref().unwrap());
        let right_size = self.get_size_or_zero(right.as_ref());
        if self.first_is_to_the_left_of_second(node, root.as_ref().unwrap()) {
            let decremented_left_size = left_size - T::funty(1);
            if self.get_size_or_zero(self.get_right_or_default(right.as_ref())) > decremented_left_size {
                self.left_rotate(root);
            } else if self.get_size_or_zero(self.get_left_or_default(right.as_ref())) > decremented_left_size {
                self.right_rotate(right);
                self.left_rotate(root);
            } else {
                self.dec_size(root.as_mut().unwrap());
                root = left;
            }
        } else if self.first_is_to_the_right_of_second(node, root.as_ref().unwrap()) {
            let decremented_right_size = right_size - T::funty(1);
            if self.get_size_or_zero(self.get_left_or_default(left.as_ref())) > decremented_right_size {
                self.right_rotate(root);
            } else if self.get_size_or_zero(self.get_right_or_default(left.as_ref())) > decremented_right_size {
                self.left_rotate(left);
                self.right_rotate(root);
            } else {
                self.dec_size(root.as_mut().unwrap());
                root = right;
            }
        } else {
            if left_size > T::funty(0) && right_size > T::funty(0) {
                let replacement;
                if left_size > right_size {
                    replacement = self.get_rightest(left.as_ref().unwrap());
                    self.detach_core(left, replacement);
                } else {
                    replacement = self.get_leftest(right.as_ref().unwrap());
                    self.detach_core(right, replacement);
                }
                self.set_left(replacement, left.take().unwrap());
                self.set_right(replacement, right.take().unwrap());
                self.set_size(replacement, left_size + right_size);
                *root = Some(replacement);
            } else if left_size > T::funty(0) {
                *root = left.take();
            } else if right_size > T::funty(0) {
                *root = right.take();
            } else {
                *root = None;
            }
            self.clear_node(node);
            return;
        }
    }
}
}

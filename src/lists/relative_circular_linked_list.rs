use crate::{LinkType, RelativeLinkedList};

/// Circular doubly-linked list with head-relative positioning.
///
/// Like [`AbsoluteCircularLinkedList`](super::AbsoluteCircularLinkedList)
/// but takes a `head` parameter to support multiple independent circular
/// lists sharing the same node storage.
///
/// All methods have default implementations — an empty `impl` block
/// is sufficient once [`RelativeLinkedList`] is implemented.
pub trait RelativeCircularLinkedList<T: LinkType>: RelativeLinkedList<T> {
    /// Inserts `new_element` immediately before `base_element` in the
    /// list identified by `head`.
    fn attach_before(&mut self, head: T, base_element: T, new_element: T) {
        let base_element_previous = self.get_previous(base_element);
        self.set_previous(new_element, base_element_previous);
        self.set_next(new_element, base_element);
        if base_element == self.get_first(head) {
            self.set_first(head, new_element);
        }
        self.set_next(base_element_previous, new_element);
        self.set_previous(base_element, new_element);
        self.inc_size(head);
    }

    /// Inserts `new_element` immediately after `base_element` in the
    /// list identified by `head`.
    fn attach_after(&mut self, head: T, base_element: T, new_element: T) {
        let base_element_next = self.get_next(base_element);
        self.set_previous(new_element, base_element);
        self.set_next(new_element, base_element_next);
        if base_element == self.get_last(head) {
            self.set_last(head, new_element);
        }
        self.set_previous(base_element_next, new_element);
        self.set_next(base_element, new_element);
        self.inc_size(head);
    }

    /// Inserts `element` as the first element of the list identified
    /// by `head`.
    fn attach_as_first(&mut self, head: T, element: T) {
        let first = self.get_first(head);
        if first == T::from_byte(0) {
            self.set_first(head, element);
            self.set_last(head, element);
            self.set_previous(element, element);
            self.set_next(element, element);
            self.inc_size(head);
        } else {
            self.attach_before(head, first, element);
        }
    }

    /// Inserts `element` as the last element of the list identified
    /// by `head`.
    fn attach_as_last(&mut self, head: T, element: T) {
        let last = self.get_last(head);
        if last == T::from_byte(0) {
            self.attach_as_first(head, element);
        } else {
            self.attach_after(head, last, element);
        }
    }

    /// Removes `element` from the list identified by `head`.
    fn detach(&mut self, head: T, element: T) {
        let element_previous = self.get_previous(element);
        let element_next = self.get_next(element);
        if element_next == element {
            self.set_first(head, T::from_byte(0));
            self.set_last(head, T::from_byte(0));
        } else {
            self.set_next(element_previous, element_next);
            self.set_previous(element_next, element_previous);
            if element == self.get_first(head) {
                self.set_first(head, element_next);
            }
            if element == self.get_last(head) {
                self.set_last(head, element_previous);
            }
        }
        self.set_previous(element, T::from_byte(0));
        self.set_next(element, T::from_byte(0));
        self.dec_size(head);
    }
}

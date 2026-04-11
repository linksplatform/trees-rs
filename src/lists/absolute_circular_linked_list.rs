use crate::{AbsoluteLinkedList, LinkType};

/// Circular doubly-linked list with absolute (direct) head/tail access.
///
/// Provides `attach_before`, `attach_after`, `attach_as_first`,
/// `attach_as_last`, and `detach` operations that maintain circular
/// links and update the head/tail/size automatically.
///
/// All methods have default implementations — an empty `impl` block
/// is sufficient once [`AbsoluteLinkedList`] is implemented.
pub trait AbsoluteCircularLinkedList<T: LinkType>: AbsoluteLinkedList<T> {
    /// Inserts `new_element` immediately before `base_element`.
    ///
    /// If `base_element` is the current first element, the first
    /// pointer is updated to `new_element`.
    fn attach_before(&mut self, base_element: T, new_element: T) {
        let base_element_previous = self.get_previous(base_element);
        self.set_previous(new_element, base_element_previous);
        self.set_next(new_element, base_element);
        if base_element == self.get_first() {
            self.set_first(new_element);
        }
        self.set_next(base_element_previous, new_element);
        self.set_previous(base_element, new_element);
        self.inc_size();
    }

    /// Inserts `new_element` immediately after `base_element`.
    ///
    /// If `base_element` is the current last element, the last
    /// pointer is updated to `new_element`.
    fn attach_after(&mut self, base_element: T, new_element: T) {
        let base_element_next = self.get_next(base_element);
        self.set_previous(new_element, base_element);
        self.set_next(new_element, base_element_next);
        if base_element == self.get_last() {
            self.set_last(new_element);
        }
        self.set_previous(base_element_next, new_element);
        self.set_next(base_element, new_element);
        self.inc_size();
    }

    /// Inserts `element` as the first element of the list.
    ///
    /// If the list is empty, `element` becomes both first and last,
    /// with its previous and next pointers pointing to itself.
    fn attach_as_first(&mut self, element: T) {
        let first = self.get_first();
        if first == T::funty(0) {
            self.set_first(element);
            self.set_last(element);
            self.set_previous(element, element);
            self.set_next(element, element);
            self.inc_size();
        } else {
            self.attach_before(first, element);
        }
    }

    /// Inserts `element` as the last element of the list.
    ///
    /// If the list is empty, delegates to [`attach_as_first`](Self::attach_as_first).
    fn attach_as_last(&mut self, element: T) {
        let last = self.get_last();
        if last == T::funty(0) {
            self.attach_as_first(element);
        } else {
            self.attach_after(last, element);
        }
    }

    /// Removes `element` from the list.
    ///
    /// Updates head/tail pointers as needed and clears the element's
    /// previous and next pointers to `T::funty(0)`.
    fn detach(&mut self, element: T) {
        let element_previous = self.get_previous(element);
        let element_next = self.get_next(element);
        if element_next == element {
            self.set_first(T::funty(0));
            self.set_last(T::funty(0));
        } else {
            self.set_next(element_previous, element_next);
            self.set_previous(element_next, element_previous);
            if element == self.get_first() {
                self.set_first(element_next);
            }
            if element == self.get_last() {
                self.set_last(element_previous);
            }
        }
        self.set_previous(element, T::funty(0));
        self.set_next(element, T::funty(0));
        self.dec_size();
    }
}

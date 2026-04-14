use crate::LinkedList;
use platform_num::LinkReference;

/// Linked list with head-relative access to first, last, and size.
///
/// Unlike [`AbsoluteLinkedList`](super::AbsoluteLinkedList), this trait
/// stores head/tail/size per `head` index, allowing multiple independent
/// lists to share the same underlying node storage.
pub trait RelativeLinkedList<T: LinkReference>: LinkedList<T> {
    /// Returns the first element of the list identified by `head`.
    fn get_first(&self, head: T) -> T;

    /// Returns the last element of the list identified by `head`.
    fn get_last(&self, head: T) -> T;

    /// Returns the size of the list identified by `head`.
    fn get_size(&self, head: T) -> T;

    /// Sets the first element of the list identified by `head`.
    fn set_first(&mut self, head: T, element: T);

    /// Sets the last element of the list identified by `head`.
    fn set_last(&mut self, head: T, element: T);

    /// Sets the size of the list identified by `head`.
    fn set_size(&mut self, head: T, size: T);

    /// Increments the size of the list identified by `head` by one.
    fn inc_size(&mut self, head: T) {
        self.set_size(head, self.get_size(head) + T::from_byte(1));
    }

    /// Decrements the size of the list identified by `head` by one.
    fn dec_size(&mut self, head: T) {
        self.set_size(head, self.get_size(head) - T::from_byte(1));
    }
}

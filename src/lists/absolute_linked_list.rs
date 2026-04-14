use crate::{LinkType, LinkedList};

/// Linked list with direct (absolute) access to the first and last
/// elements and a size counter.
///
/// This is typically used when a single list instance owns its own
/// head/tail/size metadata. For multiple lists sharing the same
/// storage, see [`RelativeLinkedList`](super::RelativeLinkedList).
pub trait AbsoluteLinkedList<T: LinkType>: LinkedList<T> {
    /// Returns the first element (head) of the list, or `T::from_byte(0)` if empty.
    fn get_first(&self) -> T;

    /// Returns the last element (tail) of the list, or `T::from_byte(0)` if empty.
    fn get_last(&self) -> T;

    /// Returns the number of elements in the list.
    fn get_size(&self) -> T;

    /// Sets the first element (head) of the list.
    fn set_first(&mut self, element: T);

    /// Sets the last element (tail) of the list.
    fn set_last(&mut self, element: T);

    /// Sets the size of the list.
    fn set_size(&mut self, size: T);

    /// Increments the list size by one.
    fn inc_size(&mut self) {
        self.set_size(self.get_size() + T::from_byte(1));
    }

    /// Decrements the list size by one.
    fn dec_size(&mut self) {
        self.set_size(self.get_size() - T::from_byte(1));
    }
}

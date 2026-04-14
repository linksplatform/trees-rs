use platform_num::LinkReference;

/// Base doubly-linked list trait providing node-level navigation.
///
/// Implementors provide the storage for previous/next pointers.
/// Higher-level traits ([`AbsoluteLinkedList`](super::AbsoluteLinkedList),
/// [`RelativeLinkedList`](super::RelativeLinkedList)) build on this to
/// add head, tail, and size tracking.
///
/// # Examples
///
/// ```
/// use platform_trees::LinkedList;
///
/// struct SimpleList { prev: Vec<usize>, next: Vec<usize> }
///
/// impl LinkedList<usize> for SimpleList {
///     fn get_previous(&self, el: usize) -> usize { self.prev[el] }
///     fn get_next(&self, el: usize) -> usize { self.next[el] }
///     fn set_previous(&mut self, el: usize, p: usize) { self.prev[el] = p; }
///     fn set_next(&mut self, el: usize, n: usize) { self.next[el] = n; }
/// }
///
/// let mut list = SimpleList {
///     prev: vec![0; 4],
///     next: vec![0; 4],
/// };
/// list.set_next(1, 2);
/// assert_eq!(list.get_next(1), 2);
/// ```
pub trait LinkedList<T: LinkReference> {
    /// Returns the previous element of `element`.
    fn get_previous(&self, element: T) -> T;

    /// Returns the next element of `element`.
    fn get_next(&self, element: T) -> T;

    /// Sets the previous pointer of `element` to `previous`.
    fn set_previous(&mut self, element: T, previous: T);

    /// Sets the next pointer of `element` to `next`.
    fn set_next(&mut self, element: T, next: T);
}

//! Comprehensive tests for 100% code coverage of platform-trees

use crate::{
    AbsoluteCircularLinkedList, AbsoluteLinkedList, LinkType, LinkedList,
    RecursiveSizeBalancedTree, RelativeCircularLinkedList, RelativeLinkedList, SizeBalancedTree,
};

// =============================================================================
// Test implementations
// =============================================================================

/// A simple node structure for testing linked lists
#[derive(Debug, Clone, Copy, Default)]
struct Node {
    prev: usize,
    next: usize,
}

/// A simple absolute linked list implementation for testing
struct TestAbsoluteList {
    nodes: Vec<Node>,
    first: usize,
    last: usize,
    size: usize,
}

impl TestAbsoluteList {
    fn new(capacity: usize) -> Self {
        let mut nodes = Vec::with_capacity(capacity + 1);
        // Index 0 is reserved as "null"
        nodes.resize(capacity + 1, Node::default());
        Self {
            nodes,
            first: 0,
            last: 0,
            size: 0,
        }
    }
}

impl LinkedList<usize> for TestAbsoluteList {
    fn get_previous(&self, element: usize) -> usize {
        self.nodes[element].prev
    }

    fn get_next(&self, element: usize) -> usize {
        self.nodes[element].next
    }

    fn set_previous(&mut self, element: usize, previous: usize) {
        self.nodes[element].prev = previous;
    }

    fn set_next(&mut self, element: usize, next: usize) {
        self.nodes[element].next = next;
    }
}

impl AbsoluteLinkedList<usize> for TestAbsoluteList {
    fn get_first(&self) -> usize {
        self.first
    }

    fn get_last(&self) -> usize {
        self.last
    }

    fn get_size(&self) -> usize {
        self.size
    }

    fn set_first(&mut self, element: usize) {
        self.first = element;
    }

    fn set_last(&mut self, element: usize) {
        self.last = element;
    }

    fn set_size(&mut self, size: usize) {
        self.size = size;
    }
}

impl AbsoluteCircularLinkedList<usize> for TestAbsoluteList {}

/// A relative linked list implementation for testing (list head stored in element)
struct TestRelativeList {
    nodes: Vec<Node>,
    // Store first/last/size for each "head" element
    heads: Vec<(usize, usize, usize)>, // (first, last, size)
}

impl TestRelativeList {
    fn new(capacity: usize) -> Self {
        let mut nodes = Vec::with_capacity(capacity + 1);
        nodes.resize(capacity + 1, Node::default());
        let mut heads = Vec::with_capacity(capacity + 1);
        heads.resize(capacity + 1, (0, 0, 0));
        Self { nodes, heads }
    }
}

impl LinkedList<usize> for TestRelativeList {
    fn get_previous(&self, element: usize) -> usize {
        self.nodes[element].prev
    }

    fn get_next(&self, element: usize) -> usize {
        self.nodes[element].next
    }

    fn set_previous(&mut self, element: usize, previous: usize) {
        self.nodes[element].prev = previous;
    }

    fn set_next(&mut self, element: usize, next: usize) {
        self.nodes[element].next = next;
    }
}

impl RelativeLinkedList<usize> for TestRelativeList {
    fn get_first(&self, head: usize) -> usize {
        self.heads[head].0
    }

    fn get_last(&self, head: usize) -> usize {
        self.heads[head].1
    }

    fn get_size(&self, head: usize) -> usize {
        self.heads[head].2
    }

    fn set_first(&mut self, head: usize, element: usize) {
        self.heads[head].0 = element;
    }

    fn set_last(&mut self, head: usize, element: usize) {
        self.heads[head].1 = element;
    }

    fn set_size(&mut self, head: usize, size: usize) {
        self.heads[head].2 = size;
    }
}

impl RelativeCircularLinkedList<usize> for TestRelativeList {}

/// A tree node structure for testing SizeBalancedTree
#[derive(Debug, Clone, Copy, Default)]
struct TreeNode {
    left: usize,
    right: usize,
    size: usize,
}

/// A simple SizeBalancedTree implementation for testing
struct TestTree {
    nodes: Vec<TreeNode>,
}

impl TestTree {
    fn new(capacity: usize) -> Self {
        let mut nodes = Vec::with_capacity(capacity + 1);
        nodes.resize(capacity + 1, TreeNode::default());
        Self { nodes }
    }
}

impl RecursiveSizeBalancedTree<usize> for TestTree {
    unsafe fn get_mut_left_reference(&mut self, node: usize) -> *mut usize {
        &mut self.nodes[node].left
    }

    unsafe fn get_mut_right_reference(&mut self, node: usize) -> *mut usize {
        &mut self.nodes[node].right
    }

    unsafe fn get_left_reference(&self, node: usize) -> *const usize {
        &self.nodes[node].left
    }

    unsafe fn get_right_reference(&self, node: usize) -> *const usize {
        &self.nodes[node].right
    }

    unsafe fn get_left(&self, node: usize) -> usize {
        self.nodes[node].left
    }

    unsafe fn get_right(&self, node: usize) -> usize {
        self.nodes[node].right
    }

    unsafe fn get_size(&self, node: usize) -> usize {
        self.nodes[node].size
    }

    unsafe fn set_left(&mut self, node: usize, left: usize) {
        self.nodes[node].left = left;
    }

    unsafe fn set_right(&mut self, node: usize, right: usize) {
        self.nodes[node].right = right;
    }

    unsafe fn set_size(&mut self, node: usize, size: usize) {
        self.nodes[node].size = size;
    }

    unsafe fn first_is_to_the_left_of_second(&self, first: usize, second: usize) -> bool {
        first < second
    }

    unsafe fn first_is_to_the_right_of_second(&self, first: usize, second: usize) -> bool {
        first > second
    }
}

impl SizeBalancedTree<usize> for TestTree {}

// =============================================================================
// LinkType trait tests
// =============================================================================

#[cfg(test)]
mod link_type_tests {
    use super::*;

    #[test]
    fn test_funty_zero() {
        assert_eq!(usize::funty(0), 0usize);
        assert_eq!(u8::funty(0), 0u8);
        assert_eq!(u16::funty(0), 0u16);
        assert_eq!(u32::funty(0), 0u32);
        assert_eq!(u64::funty(0), 0u64);
    }

    #[test]
    fn test_funty_one() {
        assert_eq!(usize::funty(1), 1usize);
        assert_eq!(u8::funty(1), 1u8);
        assert_eq!(u16::funty(1), 1u16);
        assert_eq!(u32::funty(1), 1u32);
        assert_eq!(u64::funty(1), 1u64);
    }

    #[test]
    fn test_funty_various_values() {
        for i in 0..=255u8 {
            assert_eq!(u8::funty(i), i);
            assert_eq!(u16::funty(i), i as u16);
            assert_eq!(u32::funty(i), i as u32);
            assert_eq!(u64::funty(i), i as u64);
            assert_eq!(usize::funty(i), i as usize);
        }
    }
}

// =============================================================================
// LinkedList trait tests
// =============================================================================

#[cfg(test)]
mod linked_list_tests {
    use super::*;

    #[test]
    fn test_get_set_previous() {
        let mut list = TestAbsoluteList::new(10);
        list.set_previous(1, 5);
        assert_eq!(list.get_previous(1), 5);
    }

    #[test]
    fn test_get_set_next() {
        let mut list = TestAbsoluteList::new(10);
        list.set_next(1, 3);
        assert_eq!(list.get_next(1), 3);
    }
}

// =============================================================================
// AbsoluteLinkedList trait tests
// =============================================================================

#[cfg(test)]
mod absolute_linked_list_tests {
    use super::*;

    #[test]
    fn test_get_set_first() {
        let mut list = TestAbsoluteList::new(10);
        list.set_first(5);
        assert_eq!(list.get_first(), 5);
    }

    #[test]
    fn test_get_set_last() {
        let mut list = TestAbsoluteList::new(10);
        list.set_last(7);
        assert_eq!(list.get_last(), 7);
    }

    #[test]
    fn test_get_set_size() {
        let mut list = TestAbsoluteList::new(10);
        list.set_size(3);
        assert_eq!(list.get_size(), 3);
    }

    #[test]
    fn test_inc_size() {
        let mut list = TestAbsoluteList::new(10);
        list.set_size(5);
        list.inc_size();
        assert_eq!(list.get_size(), 6);
    }

    #[test]
    fn test_dec_size() {
        let mut list = TestAbsoluteList::new(10);
        list.set_size(5);
        list.dec_size();
        assert_eq!(list.get_size(), 4);
    }
}

// =============================================================================
// AbsoluteCircularLinkedList trait tests
// =============================================================================

#[cfg(test)]
mod absolute_circular_linked_list_tests {
    use super::*;

    #[test]
    fn test_attach_as_first_empty_list() {
        let mut list = TestAbsoluteList::new(10);

        list.attach_as_first(1);

        assert_eq!(list.get_first(), 1);
        assert_eq!(list.get_last(), 1);
        assert_eq!(list.get_size(), 1);
        assert_eq!(list.get_previous(1), 1); // Points to itself
        assert_eq!(list.get_next(1), 1); // Points to itself
    }

    #[test]
    fn test_attach_as_first_non_empty_list() {
        let mut list = TestAbsoluteList::new(10);

        list.attach_as_first(1);
        list.attach_as_first(2);

        assert_eq!(list.get_first(), 2);
        assert_eq!(list.get_last(), 1);
        assert_eq!(list.get_size(), 2);
    }

    #[test]
    fn test_attach_as_last_empty_list() {
        let mut list = TestAbsoluteList::new(10);

        list.attach_as_last(1);

        assert_eq!(list.get_first(), 1);
        assert_eq!(list.get_last(), 1);
        assert_eq!(list.get_size(), 1);
    }

    #[test]
    fn test_attach_as_last_non_empty_list() {
        let mut list = TestAbsoluteList::new(10);

        list.attach_as_last(1);
        list.attach_as_last(2);

        assert_eq!(list.get_first(), 1);
        assert_eq!(list.get_last(), 2);
        assert_eq!(list.get_size(), 2);
    }

    #[test]
    fn test_attach_before() {
        let mut list = TestAbsoluteList::new(10);

        // Create list: 1 <-> 2
        list.attach_as_first(1);
        list.attach_as_last(2);

        // Insert 3 before 2: 1 <-> 3 <-> 2
        list.attach_before(2, 3);

        assert_eq!(list.get_first(), 1);
        assert_eq!(list.get_last(), 2);
        assert_eq!(list.get_size(), 3);
        assert_eq!(list.get_next(1), 3);
        assert_eq!(list.get_next(3), 2);
        assert_eq!(list.get_previous(2), 3);
    }

    #[test]
    fn test_attach_before_first() {
        let mut list = TestAbsoluteList::new(10);

        list.attach_as_first(1);
        list.attach_before(1, 2);

        assert_eq!(list.get_first(), 2);
        assert_eq!(list.get_last(), 1);
        assert_eq!(list.get_size(), 2);
    }

    #[test]
    fn test_attach_after() {
        let mut list = TestAbsoluteList::new(10);

        // Create list: 1 <-> 2
        list.attach_as_first(1);
        list.attach_as_last(2);

        // Insert 3 after 1: 1 <-> 3 <-> 2
        list.attach_after(1, 3);

        assert_eq!(list.get_first(), 1);
        assert_eq!(list.get_last(), 2);
        assert_eq!(list.get_size(), 3);
        assert_eq!(list.get_next(1), 3);
        assert_eq!(list.get_next(3), 2);
    }

    #[test]
    fn test_attach_after_last() {
        let mut list = TestAbsoluteList::new(10);

        list.attach_as_first(1);
        list.attach_after(1, 2);

        assert_eq!(list.get_first(), 1);
        assert_eq!(list.get_last(), 2);
        assert_eq!(list.get_size(), 2);
    }

    #[test]
    fn test_detach_single_element() {
        let mut list = TestAbsoluteList::new(10);

        list.attach_as_first(1);
        list.detach(1);

        assert_eq!(list.get_first(), 0);
        assert_eq!(list.get_last(), 0);
        assert_eq!(list.get_size(), 0);
        assert_eq!(list.get_previous(1), 0);
        assert_eq!(list.get_next(1), 0);
    }

    #[test]
    fn test_detach_first_element() {
        let mut list = TestAbsoluteList::new(10);

        list.attach_as_first(1);
        list.attach_as_last(2);
        list.attach_as_last(3);

        list.detach(1);

        assert_eq!(list.get_first(), 2);
        assert_eq!(list.get_last(), 3);
        assert_eq!(list.get_size(), 2);
    }

    #[test]
    fn test_detach_last_element() {
        let mut list = TestAbsoluteList::new(10);

        list.attach_as_first(1);
        list.attach_as_last(2);
        list.attach_as_last(3);

        list.detach(3);

        assert_eq!(list.get_first(), 1);
        assert_eq!(list.get_last(), 2);
        assert_eq!(list.get_size(), 2);
    }

    #[test]
    fn test_detach_middle_element() {
        let mut list = TestAbsoluteList::new(10);

        list.attach_as_first(1);
        list.attach_as_last(2);
        list.attach_as_last(3);

        list.detach(2);

        assert_eq!(list.get_first(), 1);
        assert_eq!(list.get_last(), 3);
        assert_eq!(list.get_size(), 2);
        assert_eq!(list.get_next(1), 3);
        assert_eq!(list.get_previous(3), 1);
    }

    #[test]
    fn test_circular_structure() {
        let mut list = TestAbsoluteList::new(10);

        list.attach_as_first(1);
        list.attach_as_last(2);
        list.attach_as_last(3);

        // Check circular structure
        assert_eq!(list.get_next(3), 1); // Last points to first
        assert_eq!(list.get_previous(1), 3); // First points back to last
    }
}

// =============================================================================
// RelativeLinkedList trait tests
// =============================================================================

#[cfg(test)]
mod relative_linked_list_tests {
    use super::*;

    #[test]
    fn test_get_set_first() {
        let mut list = TestRelativeList::new(10);
        list.set_first(1, 5);
        assert_eq!(list.get_first(1), 5);
    }

    #[test]
    fn test_get_set_last() {
        let mut list = TestRelativeList::new(10);
        list.set_last(1, 7);
        assert_eq!(list.get_last(1), 7);
    }

    #[test]
    fn test_get_set_size() {
        let mut list = TestRelativeList::new(10);
        list.set_size(1, 3);
        assert_eq!(list.get_size(1), 3);
    }

    #[test]
    fn test_inc_size() {
        let mut list = TestRelativeList::new(10);
        list.set_size(1, 5);
        list.inc_size(1);
        assert_eq!(list.get_size(1), 6);
    }

    #[test]
    fn test_dec_size() {
        let mut list = TestRelativeList::new(10);
        list.set_size(1, 5);
        list.dec_size(1);
        assert_eq!(list.get_size(1), 4);
    }

    #[test]
    fn test_multiple_heads() {
        let mut list = TestRelativeList::new(10);

        list.set_first(1, 2);
        list.set_first(2, 3);

        assert_eq!(list.get_first(1), 2);
        assert_eq!(list.get_first(2), 3);
    }
}

// =============================================================================
// RelativeCircularLinkedList trait tests
// =============================================================================

#[cfg(test)]
mod relative_circular_linked_list_tests {
    use super::*;

    const HEAD: usize = 1;

    #[test]
    fn test_attach_as_first_empty_list() {
        let mut list = TestRelativeList::new(10);

        list.attach_as_first(HEAD, 2);

        assert_eq!(list.get_first(HEAD), 2);
        assert_eq!(list.get_last(HEAD), 2);
        assert_eq!(list.get_size(HEAD), 1);
        assert_eq!(list.get_previous(2), 2);
        assert_eq!(list.get_next(2), 2);
    }

    #[test]
    fn test_attach_as_first_non_empty_list() {
        let mut list = TestRelativeList::new(10);

        list.attach_as_first(HEAD, 2);
        list.attach_as_first(HEAD, 3);

        assert_eq!(list.get_first(HEAD), 3);
        assert_eq!(list.get_last(HEAD), 2);
        assert_eq!(list.get_size(HEAD), 2);
    }

    #[test]
    fn test_attach_as_last_empty_list() {
        let mut list = TestRelativeList::new(10);

        list.attach_as_last(HEAD, 2);

        assert_eq!(list.get_first(HEAD), 2);
        assert_eq!(list.get_last(HEAD), 2);
        assert_eq!(list.get_size(HEAD), 1);
    }

    #[test]
    fn test_attach_as_last_non_empty_list() {
        let mut list = TestRelativeList::new(10);

        list.attach_as_last(HEAD, 2);
        list.attach_as_last(HEAD, 3);

        assert_eq!(list.get_first(HEAD), 2);
        assert_eq!(list.get_last(HEAD), 3);
        assert_eq!(list.get_size(HEAD), 2);
    }

    #[test]
    fn test_attach_before() {
        let mut list = TestRelativeList::new(10);

        list.attach_as_first(HEAD, 2);
        list.attach_as_last(HEAD, 3);
        list.attach_before(HEAD, 3, 4);

        assert_eq!(list.get_first(HEAD), 2);
        assert_eq!(list.get_last(HEAD), 3);
        assert_eq!(list.get_size(HEAD), 3);
        assert_eq!(list.get_next(2), 4);
        assert_eq!(list.get_next(4), 3);
    }

    #[test]
    fn test_attach_before_first() {
        let mut list = TestRelativeList::new(10);

        list.attach_as_first(HEAD, 2);
        list.attach_before(HEAD, 2, 3);

        assert_eq!(list.get_first(HEAD), 3);
        assert_eq!(list.get_last(HEAD), 2);
        assert_eq!(list.get_size(HEAD), 2);
    }

    #[test]
    fn test_attach_after() {
        let mut list = TestRelativeList::new(10);

        list.attach_as_first(HEAD, 2);
        list.attach_as_last(HEAD, 3);
        list.attach_after(HEAD, 2, 4);

        assert_eq!(list.get_first(HEAD), 2);
        assert_eq!(list.get_last(HEAD), 3);
        assert_eq!(list.get_size(HEAD), 3);
        assert_eq!(list.get_next(2), 4);
        assert_eq!(list.get_next(4), 3);
    }

    #[test]
    fn test_attach_after_last() {
        let mut list = TestRelativeList::new(10);

        list.attach_as_first(HEAD, 2);
        list.attach_after(HEAD, 2, 3);

        assert_eq!(list.get_first(HEAD), 2);
        assert_eq!(list.get_last(HEAD), 3);
        assert_eq!(list.get_size(HEAD), 2);
    }

    #[test]
    fn test_detach_single_element() {
        let mut list = TestRelativeList::new(10);

        list.attach_as_first(HEAD, 2);
        list.detach(HEAD, 2);

        assert_eq!(list.get_first(HEAD), 0);
        assert_eq!(list.get_last(HEAD), 0);
        assert_eq!(list.get_size(HEAD), 0);
        assert_eq!(list.get_previous(2), 0);
        assert_eq!(list.get_next(2), 0);
    }

    #[test]
    fn test_detach_first_element() {
        let mut list = TestRelativeList::new(10);

        list.attach_as_first(HEAD, 2);
        list.attach_as_last(HEAD, 3);
        list.attach_as_last(HEAD, 4);

        list.detach(HEAD, 2);

        assert_eq!(list.get_first(HEAD), 3);
        assert_eq!(list.get_last(HEAD), 4);
        assert_eq!(list.get_size(HEAD), 2);
    }

    #[test]
    fn test_detach_last_element() {
        let mut list = TestRelativeList::new(10);

        list.attach_as_first(HEAD, 2);
        list.attach_as_last(HEAD, 3);
        list.attach_as_last(HEAD, 4);

        list.detach(HEAD, 4);

        assert_eq!(list.get_first(HEAD), 2);
        assert_eq!(list.get_last(HEAD), 3);
        assert_eq!(list.get_size(HEAD), 2);
    }

    #[test]
    fn test_detach_middle_element() {
        let mut list = TestRelativeList::new(10);

        list.attach_as_first(HEAD, 2);
        list.attach_as_last(HEAD, 3);
        list.attach_as_last(HEAD, 4);

        list.detach(HEAD, 3);

        assert_eq!(list.get_first(HEAD), 2);
        assert_eq!(list.get_last(HEAD), 4);
        assert_eq!(list.get_size(HEAD), 2);
        assert_eq!(list.get_next(2), 4);
        assert_eq!(list.get_previous(4), 2);
    }

    #[test]
    fn test_multiple_lists() {
        let mut list = TestRelativeList::new(20);

        const HEAD1: usize = 1;
        const HEAD2: usize = 10;

        list.attach_as_first(HEAD1, 2);
        list.attach_as_last(HEAD1, 3);

        list.attach_as_first(HEAD2, 11);
        list.attach_as_last(HEAD2, 12);

        assert_eq!(list.get_first(HEAD1), 2);
        assert_eq!(list.get_last(HEAD1), 3);
        assert_eq!(list.get_size(HEAD1), 2);

        assert_eq!(list.get_first(HEAD2), 11);
        assert_eq!(list.get_last(HEAD2), 12);
        assert_eq!(list.get_size(HEAD2), 2);
    }
}

// =============================================================================
// SizeBalancedTree trait tests
// =============================================================================

#[cfg(test)]
mod size_balanced_tree_tests {
    use super::*;

    #[test]
    fn test_get_set_left() {
        let mut tree = TestTree::new(10);
        unsafe {
            tree.set_left(1, 2);
            assert_eq!(tree.get_left(1), 2);
        }
    }

    #[test]
    fn test_get_set_right() {
        let mut tree = TestTree::new(10);
        unsafe {
            tree.set_right(1, 3);
            assert_eq!(tree.get_right(1), 3);
        }
    }

    #[test]
    fn test_get_set_size() {
        let mut tree = TestTree::new(10);
        unsafe {
            tree.set_size(1, 5);
            assert_eq!(tree.get_size(1), 5);
        }
    }

    #[test]
    fn test_get_left_or_default_with_zero() {
        let tree = TestTree::new(10);
        unsafe {
            assert_eq!(tree.get_left_or_default(0), 0);
        }
    }

    #[test]
    fn test_get_left_or_default_with_node() {
        let mut tree = TestTree::new(10);
        unsafe {
            tree.set_left(1, 2);
            assert_eq!(tree.get_left_or_default(1), 2);
        }
    }

    #[test]
    fn test_get_right_or_default_with_zero() {
        let tree = TestTree::new(10);
        unsafe {
            assert_eq!(tree.get_right_or_default(0), 0);
        }
    }

    #[test]
    fn test_get_right_or_default_with_node() {
        let mut tree = TestTree::new(10);
        unsafe {
            tree.set_right(1, 3);
            assert_eq!(tree.get_right_or_default(1), 3);
        }
    }

    #[test]
    fn test_get_size_or_zero_with_zero() {
        let tree = TestTree::new(10);
        unsafe {
            assert_eq!(tree.get_size_or_zero(0), 0);
        }
    }

    #[test]
    fn test_get_size_or_zero_with_node() {
        let mut tree = TestTree::new(10);
        unsafe {
            tree.set_size(1, 5);
            assert_eq!(tree.get_size_or_zero(1), 5);
        }
    }

    #[test]
    fn test_inc_size() {
        let mut tree = TestTree::new(10);
        unsafe {
            tree.set_size(1, 5);
            tree.inc_size(1);
            assert_eq!(tree.get_size(1), 6);
        }
    }

    #[test]
    fn test_dec_size() {
        let mut tree = TestTree::new(10);
        unsafe {
            tree.set_size(1, 5);
            tree.dec_size(1);
            assert_eq!(tree.get_size(1), 4);
        }
    }

    #[test]
    fn test_get_left_size() {
        let mut tree = TestTree::new(10);
        unsafe {
            tree.set_left(1, 2);
            tree.set_size(2, 3);
            assert_eq!(tree.get_left_size(1), 3);
        }
    }

    #[test]
    fn test_get_right_size() {
        let mut tree = TestTree::new(10);
        unsafe {
            tree.set_right(1, 3);
            tree.set_size(3, 5);
            assert_eq!(tree.get_right_size(1), 5);
        }
    }

    #[test]
    fn test_fix_size() {
        let mut tree = TestTree::new(10);
        unsafe {
            // Create tree: 1 -> (2, 3)
            tree.set_left(1, 2);
            tree.set_right(1, 3);
            tree.set_size(2, 1);
            tree.set_size(3, 2);

            tree.fix_size(1);
            assert_eq!(tree.get_size(1), 4); // 1 + 2 + 1
        }
    }

    #[test]
    fn test_left_rotate_core() {
        let mut tree = TestTree::new(10);
        unsafe {
            // Initial: 1 -> (_, 2) where 2 -> (3, _)
            tree.set_right(1, 2);
            tree.set_left(2, 3);
            tree.set_size(1, 3);
            tree.set_size(2, 2);
            tree.set_size(3, 1);

            let new_root = tree.left_rotate_core(1);

            assert_eq!(new_root, 2);
            assert_eq!(tree.get_left(2), 1);
            assert_eq!(tree.get_right(1), 3);
            assert_eq!(tree.get_size(2), 3);
        }
    }

    #[test]
    fn test_left_rotate() {
        let mut tree = TestTree::new(10);
        unsafe {
            tree.set_right(1, 2);
            tree.set_left(2, 3);
            tree.set_size(1, 3);
            tree.set_size(2, 2);
            tree.set_size(3, 1);

            let mut root: usize = 1;
            tree.left_rotate(&mut root);

            assert_eq!(root, 2);
        }
    }

    #[test]
    fn test_right_rotate_core() {
        let mut tree = TestTree::new(10);
        unsafe {
            // Initial: 1 -> (2, _) where 2 -> (_, 3)
            tree.set_left(1, 2);
            tree.set_right(2, 3);
            tree.set_size(1, 3);
            tree.set_size(2, 2);
            tree.set_size(3, 1);

            let new_root = tree.right_rotate_core(1);

            assert_eq!(new_root, 2);
            assert_eq!(tree.get_right(2), 1);
            assert_eq!(tree.get_left(1), 3);
            assert_eq!(tree.get_size(2), 3);
        }
    }

    #[test]
    fn test_right_rotate() {
        let mut tree = TestTree::new(10);
        unsafe {
            tree.set_left(1, 2);
            tree.set_right(2, 3);
            tree.set_size(1, 3);
            tree.set_size(2, 2);
            tree.set_size(3, 1);

            let mut root: usize = 1;
            tree.right_rotate(&mut root);

            assert_eq!(root, 2);
        }
    }

    #[test]
    fn test_get_rightest() {
        let mut tree = TestTree::new(10);
        unsafe {
            // Create tree: 1 -> (_, 2) where 2 -> (_, 3)
            tree.set_right(1, 2);
            tree.set_right(2, 3);

            assert_eq!(tree.get_rightest(1), 3);
        }
    }

    #[test]
    fn test_get_leftest() {
        let mut tree = TestTree::new(10);
        unsafe {
            // Create tree: 3 -> (2, _) where 2 -> (1, _)
            tree.set_left(3, 2);
            tree.set_left(2, 1);

            assert_eq!(tree.get_leftest(3), 1);
        }
    }

    #[test]
    fn test_get_next() {
        let mut tree = TestTree::new(10);
        unsafe {
            // Node 1 has right child 2 which has left child 3
            tree.set_right(1, 2);
            tree.set_left(2, 3);

            assert_eq!(tree.get_next(1), 3);
        }
    }

    #[test]
    fn test_get_previous() {
        let mut tree = TestTree::new(10);
        unsafe {
            // Node 3 has left child 2 which has right child 1
            tree.set_left(3, 2);
            tree.set_right(2, 1);

            assert_eq!(tree.get_previous(3), 1);
        }
    }

    #[test]
    fn test_contains_found() {
        let mut tree = TestTree::new(10);
        unsafe {
            // Create BST: 5 -> (3, 7)
            tree.set_left(5, 3);
            tree.set_right(5, 7);

            assert!(tree.contains(5, 5));
            assert!(tree.contains(3, 5));
            assert!(tree.contains(7, 5));
        }
    }

    #[test]
    fn test_contains_not_found() {
        let mut tree = TestTree::new(10);
        unsafe {
            // Create BST: 5 -> (3, 7)
            tree.set_left(5, 3);
            tree.set_right(5, 7);

            assert!(!tree.contains(2, 5));
            assert!(!tree.contains(4, 5));
            assert!(!tree.contains(8, 5));
        }
    }

    #[test]
    fn test_contains_empty_tree() {
        let tree = TestTree::new(10);
        unsafe {
            assert!(!tree.contains(1, 0));
        }
    }

    #[test]
    fn test_clear_node() {
        let mut tree = TestTree::new(10);
        unsafe {
            tree.set_left(1, 2);
            tree.set_right(1, 3);
            tree.set_size(1, 5);

            tree.clear_node(1);

            assert_eq!(tree.get_left(1), 0);
            assert_eq!(tree.get_right(1), 0);
            assert_eq!(tree.get_size(1), 0);
        }
    }

    #[test]
    fn test_first_is_to_the_left_of_second() {
        let tree = TestTree::new(10);
        unsafe {
            assert!(tree.first_is_to_the_left_of_second(1, 2));
            assert!(!tree.first_is_to_the_left_of_second(2, 1));
            assert!(!tree.first_is_to_the_left_of_second(1, 1));
        }
    }

    #[test]
    fn test_first_is_to_the_right_of_second() {
        let tree = TestTree::new(10);
        unsafe {
            assert!(tree.first_is_to_the_right_of_second(2, 1));
            assert!(!tree.first_is_to_the_right_of_second(1, 2));
            assert!(!tree.first_is_to_the_right_of_second(1, 1));
        }
    }

    #[test]
    fn test_get_mut_left_reference() {
        let mut tree = TestTree::new(10);
        unsafe {
            let ptr = tree.get_mut_left_reference(1);
            *ptr = 5;
            assert_eq!(tree.get_left(1), 5);
        }
    }

    #[test]
    fn test_get_mut_right_reference() {
        let mut tree = TestTree::new(10);
        unsafe {
            let ptr = tree.get_mut_right_reference(1);
            *ptr = 7;
            assert_eq!(tree.get_right(1), 7);
        }
    }

    #[test]
    fn test_get_left_reference() {
        let mut tree = TestTree::new(10);
        unsafe {
            tree.set_left(1, 3);
            let ptr = tree.get_left_reference(1);
            assert_eq!(*ptr, 3);
        }
    }

    #[test]
    fn test_get_right_reference() {
        let mut tree = TestTree::new(10);
        unsafe {
            tree.set_right(1, 4);
            let ptr = tree.get_right_reference(1);
            assert_eq!(*ptr, 4);
        }
    }
}

// =============================================================================
// NoRecurSizeBalancedTree trait tests
// =============================================================================

#[cfg(test)]
mod no_recur_size_balanced_tree_tests {
    use super::*;

    #[test]
    fn test_attach_to_empty_tree() {
        let mut tree = TestTree::new(10);
        let mut root: usize = 0;

        unsafe {
            tree.attach(&mut root, 5);
        }

        assert_eq!(root, 5);
        unsafe {
            assert_eq!(tree.get_size(5), 1);
        }
    }

    #[test]
    fn test_attach_single_node_to_left() {
        let mut tree = TestTree::new(10);
        let mut root: usize = 0;

        unsafe {
            tree.attach(&mut root, 5);
            tree.attach(&mut root, 3);
        }

        unsafe {
            assert_eq!(tree.get_left(5), 3);
            assert_eq!(tree.get_size(5), 2);
            assert_eq!(tree.get_size(3), 1);
        }
    }

    #[test]
    fn test_attach_single_node_to_right() {
        let mut tree = TestTree::new(10);
        let mut root: usize = 0;

        unsafe {
            tree.attach(&mut root, 5);
            tree.attach(&mut root, 7);
        }

        unsafe {
            assert_eq!(tree.get_right(5), 7);
            assert_eq!(tree.get_size(5), 2);
            assert_eq!(tree.get_size(7), 1);
        }
    }

    #[test]
    fn test_attach_multiple_nodes() {
        let mut tree = TestTree::new(20);
        let mut root: usize = 0;

        unsafe {
            tree.attach(&mut root, 10);
            tree.attach(&mut root, 5);
            tree.attach(&mut root, 15);
            tree.attach(&mut root, 3);
            tree.attach(&mut root, 7);
        }

        unsafe {
            assert!(tree.contains(10, root));
            assert!(tree.contains(5, root));
            assert!(tree.contains(15, root));
            assert!(tree.contains(3, root));
            assert!(tree.contains(7, root));
            assert_eq!(tree.get_size(root), 5);
        }
    }

    #[test]
    fn test_detach_from_single_node_tree() {
        let mut tree = TestTree::new(10);
        let mut root: usize = 0;

        unsafe {
            tree.attach(&mut root, 5);
            tree.detach(&mut root, 5);
        }

        assert_eq!(root, 0);
    }

    #[test]
    fn test_detach_leaf_node() {
        let mut tree = TestTree::new(20);
        let mut root: usize = 0;

        unsafe {
            tree.attach(&mut root, 10);
            tree.attach(&mut root, 5);
            tree.attach(&mut root, 15);

            tree.detach(&mut root, 5);

            assert!(!tree.contains(5, root));
            assert!(tree.contains(10, root));
            assert!(tree.contains(15, root));
        }
    }

    #[test]
    fn test_detach_node_with_one_child() {
        let mut tree = TestTree::new(20);
        let mut root: usize = 0;

        unsafe {
            tree.attach(&mut root, 10);
            tree.attach(&mut root, 5);
            tree.attach(&mut root, 3);

            tree.detach(&mut root, 5);

            assert!(!tree.contains(5, root));
            assert!(tree.contains(10, root));
            assert!(tree.contains(3, root));
        }
    }

    #[test]
    fn test_detach_node_with_two_children() {
        let mut tree = TestTree::new(20);
        let mut root: usize = 0;

        unsafe {
            tree.attach(&mut root, 10);
            tree.attach(&mut root, 5);
            tree.attach(&mut root, 15);
            tree.attach(&mut root, 3);
            tree.attach(&mut root, 7);

            tree.detach(&mut root, 5);

            assert!(!tree.contains(5, root));
            assert!(tree.contains(10, root));
            assert!(tree.contains(15, root));
            assert!(tree.contains(3, root));
            assert!(tree.contains(7, root));
        }
    }

    #[test]
    fn test_detach_root() {
        let mut tree = TestTree::new(20);
        let mut root: usize = 0;

        unsafe {
            tree.attach(&mut root, 10);
            tree.attach(&mut root, 5);
            tree.attach(&mut root, 15);

            tree.detach(&mut root, 10);

            assert!(!tree.contains(10, root));
            assert!(tree.contains(5, root));
            assert!(tree.contains(15, root));
            assert_ne!(root, 0);
        }
    }

    #[test]
    fn test_attach_and_detach_sequence() {
        let mut tree = TestTree::new(30);
        let mut root: usize = 0;

        unsafe {
            // Build tree
            for i in [10, 5, 15, 3, 7, 12, 17] {
                tree.attach(&mut root, i);
            }

            // Verify all nodes present
            for i in [10, 5, 15, 3, 7, 12, 17] {
                assert!(tree.contains(i, root), "Node {} should be present", i);
            }

            // Remove some nodes
            tree.detach(&mut root, 3);
            tree.detach(&mut root, 17);
            tree.detach(&mut root, 10);

            // Verify correct nodes present
            assert!(!tree.contains(3, root));
            assert!(!tree.contains(17, root));
            assert!(!tree.contains(10, root));
            assert!(tree.contains(5, root));
            assert!(tree.contains(15, root));
            assert!(tree.contains(7, root));
            assert!(tree.contains(12, root));
        }
    }

    #[test]
    fn test_tree_balancing() {
        let mut tree = TestTree::new(20);
        let mut root: usize = 0;

        unsafe {
            // Insert in order that would cause imbalance without rotations
            for i in 1..=10 {
                tree.attach(&mut root, i);
            }

            // All nodes should be reachable
            for i in 1..=10 {
                assert!(tree.contains(i, root), "Node {} should be present", i);
            }

            assert_eq!(tree.get_size(root), 10);
        }
    }

    #[test]
    fn test_detach_all_nodes() {
        let mut tree = TestTree::new(20);
        let mut root: usize = 0;

        unsafe {
            for i in 1..=5 {
                tree.attach(&mut root, i);
            }

            for i in 1..=5 {
                tree.detach(&mut root, i);
            }

            assert_eq!(root, 0);
        }
    }

    // Additional edge case tests for full coverage

    #[test]
    fn test_attach_left_right_rotation_edge_case() {
        // Tests the path where we need a left-right double rotation on left side
        let mut tree = TestTree::new(30);
        let mut root: usize = 0;

        unsafe {
            // Build a tree that will trigger left-right rotation
            // Start with 10, then 5, then insert 8 (between 5 and 10)
            tree.attach(&mut root, 10);
            tree.attach(&mut root, 5);
            tree.attach(&mut root, 8); // This goes right of 5, should trigger LR rotation

            assert!(tree.contains(10, root));
            assert!(tree.contains(5, root));
            assert!(tree.contains(8, root));
            assert_eq!(tree.get_size(root), 3);
        }
    }

    #[test]
    fn test_attach_descend_left_without_rotation() {
        // Tests the case where we descend left without rotation
        let mut tree = TestTree::new(40);
        let mut root: usize = 0;

        unsafe {
            // Build a more balanced tree first
            tree.attach(&mut root, 20);
            tree.attach(&mut root, 10);
            tree.attach(&mut root, 30);
            tree.attach(&mut root, 25);
            tree.attach(&mut root, 35);
            // Now insert something that needs to go left but doesn't trigger rotation
            tree.attach(&mut root, 5);
            tree.attach(&mut root, 2);

            assert!(tree.contains(2, root));
            assert_eq!(tree.get_size(root), 7);
        }
    }

    #[test]
    fn test_attach_special_case_left_right_empty() {
        // Tests the special case in attach_core where left_right_size == 0 and right_size == 0
        let mut tree = TestTree::new(20);
        let mut root: usize = 0;

        unsafe {
            // Create specific tree structure
            tree.attach(&mut root, 10);
            tree.attach(&mut root, 5);
            // Now insert 7 which is right of 5 but left of 10
            // With just 2 nodes, this triggers the special case
            tree.attach(&mut root, 7);

            assert!(tree.contains(10, root));
            assert!(tree.contains(5, root));
            assert!(tree.contains(7, root));
        }
    }

    #[test]
    fn test_attach_right_left_rotation_edge_case() {
        // Tests the path where we need a right-left double rotation on right side
        let mut tree = TestTree::new(30);
        let mut root: usize = 0;

        unsafe {
            // Build a tree that will trigger right-left rotation
            tree.attach(&mut root, 10);
            tree.attach(&mut root, 20);
            tree.attach(&mut root, 15); // Between 10 and 20, should trigger RL rotation

            assert!(tree.contains(10, root));
            assert!(tree.contains(15, root));
            assert!(tree.contains(20, root));
            assert_eq!(tree.get_size(root), 3);
        }
    }

    #[test]
    fn test_attach_descend_right_without_rotation() {
        // Tests the case where we descend right without rotation
        let mut tree = TestTree::new(40);
        let mut root: usize = 0;

        unsafe {
            // Build a balanced tree
            tree.attach(&mut root, 20);
            tree.attach(&mut root, 30);
            tree.attach(&mut root, 10);
            tree.attach(&mut root, 5);
            tree.attach(&mut root, 15);
            // Insert on the right side, descending
            tree.attach(&mut root, 35);
            tree.attach(&mut root, 40);

            assert!(tree.contains(40, root));
            assert_eq!(tree.get_size(root), 7);
        }
    }

    #[test]
    fn test_detach_with_double_rotation_left() {
        // Tests detach causing double rotation on left side
        let mut tree = TestTree::new(40);
        let mut root: usize = 0;

        unsafe {
            // Build larger tree
            for i in [20, 10, 30, 5, 15, 25, 35, 3, 8, 13, 17] {
                tree.attach(&mut root, i);
            }

            // Detach to trigger specific paths
            tree.detach(&mut root, 3);
            tree.detach(&mut root, 5);
            tree.detach(&mut root, 8);

            assert!(!tree.contains(3, root));
            assert!(!tree.contains(5, root));
            assert!(!tree.contains(8, root));
        }
    }

    #[test]
    fn test_detach_with_double_rotation_right() {
        // Tests detach causing double rotation on right side
        let mut tree = TestTree::new(50);
        let mut root: usize = 0;

        unsafe {
            // Build larger tree
            for i in [20, 10, 30, 5, 15, 25, 35, 32, 38, 28, 27] {
                tree.attach(&mut root, i);
            }

            // Detach to trigger specific paths
            tree.detach(&mut root, 35);
            tree.detach(&mut root, 38);
            tree.detach(&mut root, 32);

            assert!(!tree.contains(35, root));
            assert!(!tree.contains(38, root));
            assert!(!tree.contains(32, root));
        }
    }

    #[test]
    fn test_detach_with_left_only_child() {
        // Tests detach where replacement comes from left subtree
        let mut tree = TestTree::new(30);
        let mut root: usize = 0;

        unsafe {
            // Create tree where left subtree is larger
            tree.attach(&mut root, 10);
            tree.attach(&mut root, 5);
            tree.attach(&mut root, 15);
            tree.attach(&mut root, 3);
            tree.attach(&mut root, 7);
            tree.attach(&mut root, 2);
            tree.attach(&mut root, 4);

            // Detach a node that will use replacement from left (larger) subtree
            tree.detach(&mut root, 5);

            assert!(!tree.contains(5, root));
            assert!(tree.contains(3, root));
            assert!(tree.contains(7, root));
        }
    }

    #[test]
    fn test_detach_node_with_left_child_only() {
        // Tests the path where we set *root = *left
        let mut tree = TestTree::new(20);
        let mut root: usize = 0;

        unsafe {
            tree.attach(&mut root, 10);
            tree.attach(&mut root, 5);
            tree.attach(&mut root, 15);
            tree.attach(&mut root, 3);

            // Remove 5, which has only left child (3)
            tree.detach(&mut root, 5);

            assert!(!tree.contains(5, root));
            assert!(tree.contains(3, root));
        }
    }

    #[test]
    fn test_large_sequential_insert() {
        // Tests inserting many nodes in ascending order
        let mut tree = TestTree::new(50);
        let mut root: usize = 0;

        unsafe {
            for i in 1..=20 {
                tree.attach(&mut root, i);
            }

            for i in 1..=20 {
                assert!(tree.contains(i, root), "Node {} missing", i);
            }
            assert_eq!(tree.get_size(root), 20);
        }
    }

    #[test]
    fn test_large_reverse_insert() {
        // Tests inserting many nodes in descending order
        let mut tree = TestTree::new(50);
        let mut root: usize = 0;

        unsafe {
            for i in (1..=20).rev() {
                tree.attach(&mut root, i);
            }

            for i in 1..=20 {
                assert!(tree.contains(i, root), "Node {} missing", i);
            }
            assert_eq!(tree.get_size(root), 20);
        }
    }

    #[test]
    fn test_alternating_insert() {
        // Tests inserting nodes in alternating pattern
        let mut tree = TestTree::new(50);
        let mut root: usize = 0;

        unsafe {
            // Insert 10, 1, 20, 2, 19, 3, 18, etc.
            let mut low = 1;
            let mut high = 20;
            for i in 0..20 {
                if i % 2 == 0 {
                    tree.attach(&mut root, high);
                    high -= 1;
                } else {
                    tree.attach(&mut root, low);
                    low += 1;
                }
            }

            for i in 1..=20 {
                assert!(tree.contains(i, root), "Node {} missing", i);
            }
        }
    }

    #[test]
    fn test_detach_from_large_tree() {
        // Tests various detach patterns on large tree
        let mut tree = TestTree::new(100);
        let mut root: usize = 0;

        unsafe {
            // Build tree with 50 nodes
            for i in 1..=50 {
                tree.attach(&mut root, i);
            }

            // Detach every other node
            for i in (1..=50).step_by(2) {
                tree.detach(&mut root, i);
            }

            // Check remaining nodes
            for i in (2..=50).step_by(2) {
                assert!(tree.contains(i, root), "Node {} should be present", i);
            }
            for i in (1..=49).step_by(2) {
                assert!(!tree.contains(i, root), "Node {} should be absent", i);
            }
        }
    }
}

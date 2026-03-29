//! Shared test implementations for integration tests.
//!
//! These test fixtures provide concrete implementations of the library's traits
//! for use in testing. They serve as clear examples of how to implement each trait.

#![allow(dead_code)]

use platform_trees::{
    AbsoluteCircularLinkedList, AbsoluteLinkedList, IterativeSizeBalancedTree, LinkedList,
    RecursiveSizeBalancedTree, RelativeCircularLinkedList, RelativeLinkedList,
};

// =============================================================================
// Test implementations
// =============================================================================

/// A simple node structure for testing linked lists
#[derive(Debug, Clone, Copy, Default)]
pub struct Node {
    pub prev: usize,
    pub next: usize,
}

/// A simple absolute linked list implementation for testing
pub struct TestAbsoluteList {
    pub nodes: Vec<Node>,
    pub first: usize,
    pub last: usize,
    pub size: usize,
}

impl TestAbsoluteList {
    pub fn new(capacity: usize) -> Self {
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
pub struct TestRelativeList {
    pub nodes: Vec<Node>,
    // Store first/last/size for each "head" element
    pub heads: Vec<(usize, usize, usize)>, // (first, last, size)
}

impl TestRelativeList {
    pub fn new(capacity: usize) -> Self {
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

/// A tree node structure for testing `SizeBalancedTree`
#[derive(Debug, Clone, Copy, Default)]
pub struct TreeNode {
    pub left: usize,
    pub right: usize,
    pub size: usize,
}

/// A simple `SizeBalancedTree` implementation for testing
pub struct TestTree {
    pub nodes: Vec<TreeNode>,
}

impl TestTree {
    pub fn new(capacity: usize) -> Self {
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

impl IterativeSizeBalancedTree<usize> for TestTree {}

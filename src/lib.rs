//! # platform-trees
//!
//! Generic traits for size-balanced binary trees and circular linked lists
//! used throughout the [LinksPlatform](https://github.com/linksplatform) ecosystem.
//!
//! This crate provides trait-based abstractions that allow downstream crates
//! (such as [`doublets`](https://crates.io/crates/doublets)) to implement
//! their own storage-backed trees and lists while reusing the balancing,
//! rotation, and list manipulation logic defined here.
//!
//! ## Traits overview
//!
//! | Trait | Description |
//! |---|---|
//! | [`LinkType`] | Deprecated alias — use [`LinkReference`] instead |
//! | [`LinkedList`] | Base doubly-linked list with `get_previous`/`get_next` |
//! | [`AbsoluteLinkedList`] | List with direct `first`/`last`/`size` access |
//! | [`RelativeLinkedList`] | List with head-relative `first`/`last`/`size` |
//! | [`AbsoluteCircularLinkedList`] | Circular list with absolute positioning |
//! | [`RelativeCircularLinkedList`] | Circular list with head-relative positioning |
//! | [`RecursiveSizeBalancedTree`] | Size-balanced BST with rotations and queries |
//! | [`IterativeSizeBalancedTree`] | Iterative `attach`/`detach` over the recursive base |
//!
//! ## Quick start
//!
//! ```rust
//! use platform_trees::{LinkedList, AbsoluteLinkedList, AbsoluteCircularLinkedList};
//! use platform_num::LinkReference;
//!
//! // 1. Define your storage
//! #[derive(Default, Clone, Copy)]
//! struct Node { prev: usize, next: usize }
//!
//! struct MyList {
//!     nodes: Vec<Node>,
//!     first: usize,
//!     last: usize,
//!     size: usize,
//! }
//!
//! // 2. Implement the required traits
//! impl LinkedList<usize> for MyList {
//!     fn get_previous(&self, el: usize) -> usize { self.nodes[el].prev }
//!     fn get_next(&self, el: usize) -> usize { self.nodes[el].next }
//!     fn set_previous(&mut self, el: usize, p: usize) { self.nodes[el].prev = p; }
//!     fn set_next(&mut self, el: usize, n: usize) { self.nodes[el].next = n; }
//! }
//!
//! impl AbsoluteLinkedList<usize> for MyList {
//!     fn get_first(&self) -> usize { self.first }
//!     fn get_last(&self) -> usize { self.last }
//!     fn get_size(&self) -> usize { self.size }
//!     fn set_first(&mut self, el: usize) { self.first = el; }
//!     fn set_last(&mut self, el: usize) { self.last = el; }
//!     fn set_size(&mut self, s: usize) { self.size = s; }
//! }
//!
//! impl AbsoluteCircularLinkedList<usize> for MyList {}
//!
//! // 3. Use the provided methods
//! let mut list = MyList {
//!     nodes: vec![Node::default(); 5],
//!     first: 0, last: 0, size: 0,
//! };
//! list.attach_as_first(1);
//! list.attach_as_last(2);
//! assert_eq!(list.get_size(), 2);
//! assert_eq!(list.get_first(), 1);
//! assert_eq!(list.get_last(), 2);
//! ```

mod link_type;
mod lists;
mod trees;

pub use link_type::LinkType;
pub use lists::{
    AbsoluteCircularLinkedList, AbsoluteLinkedList, LinkedList, RelativeCircularLinkedList,
    RelativeLinkedList,
};
pub use platform_num::LinkReference;

pub use trees::{IterativeSizeBalancedTree, RecursiveSizeBalancedTree};

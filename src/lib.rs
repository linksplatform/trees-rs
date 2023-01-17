#![feature(default_free_fn)]
// fixme: #![no_std]
#![deny(clippy::all, clippy::perf)]
//#![deny(unused_must_use)]
#![allow(clippy::unit_arg)]

mod lists;
mod trees;

pub use lists::{
    AbsoluteCircularLinkedList, AbsoluteLinkedList, LinkedList, RelativeCircularLinkedList,
    RelativeLinkedList,
};

pub use trees::{new, new_v2, NoRecurSzbTree, SzbTree};

#[cfg(any(new_api, test))]
pub mod inner;

#[cfg(any(new_api, test))]
pub use inner::{BTree, New, NewV2, OldStore, Store};

#![feature(let_else)]
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

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

pub use trees::{NewNewNoRecur, NewNewTree, NewNoRecur, NewTree, NoRecurSzbTree, Node, SzbTree};

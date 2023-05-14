#![feature(default_free_fn)]
// fixme: #![no_std]
#![deny(unused_must_use)]

mod lists;
mod trees;

pub use {
    lists::{
        AbsoluteCircularLinkedList, AbsoluteLinkedList, LinkedList, RelativeCircularLinkedList,
        RelativeLinkedList,
    },
    trees::{new, new_v2, NoRecurSzbTree, SzbTree},
};

#[cfg(any(new_api, test))]
pub mod inner;

#[cfg(any(new_api, test))]
pub use inner::{BTree, New, NewV2, OldStore, Store};

macro_rules! named {
    ($($name:ident => $val:expr)*) => {
        $(
            fn $name() -> Self {
                Self::from_addr($val)
            }
        )*
    };
}

// bridge to old api
pub trait LinkType: platform_data::LinkType + funty::Unsigned {
    named! {
        zero => 0
        one => 1
        two => 2
    }
}

impl<All> LinkType for All where All: platform_data::LinkType + funty::Unsigned {}

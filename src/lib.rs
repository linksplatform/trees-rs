#![feature(default_free_fn)]
#![feature(let_else)]

// fixme: #![no_std]
#![deny(unused_must_use)]

mod lists;
mod trees;

pub use {
    lists::{
        AbsoluteCircularLinkedList, AbsoluteLinkedList, LinkedList, RelativeCircularLinkedList,
        RelativeLinkedList,
    },
    trees::{new_v2, NoRecurSzbTree, SzbTree},
};

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

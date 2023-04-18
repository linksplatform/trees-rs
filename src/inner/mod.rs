pub use store::{New, NewV2, OldStore, Store};

mod store;

pub trait BTree {
    type Item;

    fn new(len: usize) -> Self;

    fn _attach(&mut self, root: &mut Option<Self::Item>, node: Self::Item);

    fn _detach(&mut self, root: &mut Option<Self::Item>, node: Self::Item);

    fn is_contains(&self, root: Self::Item, node: Self::Item) -> bool;

    fn reset(&mut self);
}

pub use store::{New, NewV2, OldStore, Store};

mod store;

pub trait BTree {
    type Item;

    fn add(&mut self, root: &mut Option<Self::Item>, node: Self::Item);

    fn remove(&mut self, root: &mut Option<Self::Item>, node: Self::Item);
}

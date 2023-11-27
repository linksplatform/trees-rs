mod no_recur_szb_tree;
mod szb_tree;

macro_rules! tri {
    ($expr:expr) => {
        match $expr {
            Some(x) => x,
            None => return false,
        }
    };
}

pub use {leaf::Leaf, no_recur_szb_tree::NoRecurSzbTree, szb_tree::SzbTree};

mod leaf;
pub mod new;

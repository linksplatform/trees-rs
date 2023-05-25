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

pub use {no_recur_szb_tree::NoRecurSzbTree, szb_tree::SzbTree};

pub mod new_v2;

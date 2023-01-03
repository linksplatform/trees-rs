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

macro_rules! fn_set {
    ($($name:ident => $set:ident: $ty:ty)*) => {
        $(
            fn $name(slice: &mut [Self::Item], idx: usize, $set: $ty) {
                Self::_get(slice, idx).map(|node| Self::_set(slice, idx, Node { $set, ..node }));
            }
        )*
    };
}

pub use no_recur_szb_tree::NoRecurSzbTree;
pub use szb_tree::SzbTree;

pub mod new;
pub mod new_v2;

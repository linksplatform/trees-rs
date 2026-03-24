use num_traits::{FromPrimitive, Unsigned};
use platform_num::Number;
use std::convert::TryFrom;

/// Extension trait providing the `funty` method for converting small integers to any `LinkType`.
pub trait LinkType: Number + Unsigned + Sized + TryFrom<u8> + FromPrimitive {
    /// Convert a small integer (u8) to Self.
    /// This is a convenience method for creating zero, one, or small constants.
    fn funty(n: u8) -> Self;
}

impl<T: Number + Unsigned + Sized + TryFrom<u8> + FromPrimitive> LinkType for T {
    #[inline]
    fn funty(n: u8) -> Self {
        T::try_from(n).unwrap_or_else(|_| {
            unreachable!("u8 conversion should always succeed for unsigned types")
        })
    }
}

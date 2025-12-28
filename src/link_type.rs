use funty::Unsigned;
use platform_num::LinkType as BaseLink;
use std::convert::TryFrom;

/// Extension trait providing the `funty` method for converting small integers to any LinkType.
pub trait LinkType: BaseLink + Unsigned + Sized + TryFrom<u8> {
    /// Convert a small integer (u8) to Self.
    /// This is a convenience method for creating zero, one, or small constants.
    fn funty(n: u8) -> Self;
}

impl<T: BaseLink + Unsigned + Sized + TryFrom<u8>> LinkType for T {
    #[inline]
    fn funty(n: u8) -> Self {
        match T::try_from(n) {
            Ok(val) => val,
            Err(_) => unreachable!("u8 conversion should always succeed for unsigned types"),
        }
    }
}

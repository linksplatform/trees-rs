use num_traits::{FromPrimitive, Unsigned};
use platform_num::Number;
use std::convert::TryFrom;

/// Trait for unsigned integer types that can serve as node indices
/// in trees and linked lists.
///
/// `LinkType` combines [`Number`], [`Unsigned`], [`TryFrom<u8>`], and
/// [`FromPrimitive`] and adds a convenience [`funty`](LinkType::funty)
/// method for creating small constants.
///
/// # Blanket implementation
///
/// Every type that satisfies the supertraits automatically implements
/// `LinkType`, so standard unsigned types (`u8`, `u16`, `u32`, `u64`,
/// `usize`) work out of the box:
///
/// ```
/// use platform_trees::LinkType;
///
/// assert_eq!(u32::funty(0), 0u32);
/// assert_eq!(u64::funty(1), 1u64);
/// assert_eq!(usize::funty(42), 42usize);
/// ```
pub trait LinkType: Number + Unsigned + Sized + TryFrom<u8> + FromPrimitive {
    /// Convert a `u8` value to `Self`.
    ///
    /// This is a convenience method used internally for zero, one,
    /// and other small constants needed by tree and list algorithms.
    ///
    /// # Examples
    ///
    /// ```
    /// use platform_trees::LinkType;
    ///
    /// let zero = usize::funty(0);
    /// let one  = u32::funty(1);
    /// assert_eq!(zero, 0);
    /// assert_eq!(one, 1);
    /// ```
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

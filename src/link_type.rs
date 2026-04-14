use platform_num::LinkReference;

/// Marker trait for unsigned integer types that can serve as node indices
/// in trees and linked lists.
///
/// Since version 0.3.0 this is a simple supertrait of
/// [`LinkReference`](platform_num::LinkReference) from `platform-num`.
/// New code should use `LinkReference` directly.
///
/// # Blanket implementation
///
/// Every type that implements `LinkReference` automatically implements
/// `LinkType`, so standard unsigned types (`u8`, `u16`, `u32`, `u64`,
/// `u128`, `usize`) work out of the box:
///
/// ```
/// use platform_trees::LinkType;
/// use platform_num::LinkReference;
///
/// assert_eq!(u32::from_byte(0), 0u32);
/// assert_eq!(u64::from_byte(1), 1u64);
/// assert_eq!(usize::from_byte(42), 42usize);
/// ```
pub trait LinkType: LinkReference {}

impl<T: LinkReference> LinkType for T {}

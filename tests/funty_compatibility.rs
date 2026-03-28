// Experiment: Verify that removing the `funty` crate dependency from trees-rs
// does NOT break the public `funty()` method API.
//
// Key finding: The `funty` crate and the `funty()` method are different things.
// - `funty` crate: provides `funty::Unsigned` trait
// - `funty()` method: a convenience method on the `LinkType` trait for creating
//   small integer values from u8
//
// PR #9 removed the `funty` crate dependency but PRESERVED the `funty()` method
// on the `LinkType` trait. The `funty()` method now uses `num_traits::Unsigned`
// instead of `funty::Unsigned` for the trait bound, but the method signature
// and behavior are identical.
//
// This test demonstrates that the `funty()` method works correctly with all
// standard unsigned integer types used in downstream crates like doublets-rs.

#[cfg(test)]
mod tests {
    use platform_trees::LinkType;

    /// Verify `T::funty(0)` works - used extensively in doublets-rs for null/zero checks.
    #[test]
    fn test_funty_zero_all_types() {
        assert_eq!(u8::funty(0), 0u8);
        assert_eq!(u16::funty(0), 0u16);
        assert_eq!(u32::funty(0), 0u32);
        assert_eq!(u64::funty(0), 0u64);
        assert_eq!(usize::funty(0), 0usize);
    }

    /// Verify `T::funty(1)` works - used in doublets-rs for range starts.
    #[test]
    fn test_funty_one_all_types() {
        assert_eq!(u8::funty(1), 1u8);
        assert_eq!(u16::funty(1), 1u16);
        assert_eq!(u32::funty(1), 1u32);
        assert_eq!(u64::funty(1), 1u64);
        assert_eq!(usize::funty(1), 1usize);
    }

    /// Verify `T::funty(2)` works - used in data-rs for `default_target_part`.
    #[test]
    fn test_funty_two_all_types() {
        assert_eq!(u8::funty(2), 2u8);
        assert_eq!(u16::funty(2), 2u16);
        assert_eq!(u32::funty(2), 2u32);
        assert_eq!(u64::funty(2), 2u64);
        assert_eq!(usize::funty(2), 2usize);
    }

    /// Verify funty works in generic context - how doublets-rs actually uses it.
    fn generic_funty_usage<T: LinkType>() -> (T, T, T) {
        (T::funty(0), T::funty(1), T::funty(2))
    }

    #[test]
    fn test_generic_funty_usage() {
        let (zero, one, two) = generic_funty_usage::<u32>();
        assert_eq!(zero, 0u32);
        assert_eq!(one, 1u32);
        assert_eq!(two, 2u32);

        let (zero, one, two) = generic_funty_usage::<u64>();
        assert_eq!(zero, 0u64);
        assert_eq!(one, 1u64);
        assert_eq!(two, 2u64);

        let (zero, one, two) = generic_funty_usage::<usize>();
        assert_eq!(zero, 0usize);
        assert_eq!(one, 1usize);
        assert_eq!(two, 2usize);
    }

    /// Verify comparison patterns used in doublets-rs.
    #[test]
    fn test_funty_comparison_patterns() {
        fn check_null<T: LinkType>(value: T) -> bool {
            value == T::funty(0)
        }

        assert!(check_null(0u32));
        assert!(!check_null(1u32));
        assert!(check_null(0u64));
        assert!(!check_null(42u64));
    }

    /// Verify arithmetic patterns used in trees-rs itself for `inc_size`/`dec_size`.
    #[test]
    fn test_funty_arithmetic_patterns() {
        fn inc_value<T: LinkType>(val: T) -> T {
            val + T::funty(1)
        }

        fn dec_value<T: LinkType>(val: T) -> T {
            val - T::funty(1)
        }

        assert_eq!(inc_value(5u32), 6u32);
        assert_eq!(dec_value(5u32), 4u32);
        assert_eq!(inc_value(0u64), 1u64);
        assert_eq!(dec_value(10u64), 9u64);
    }
}

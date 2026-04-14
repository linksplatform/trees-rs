// Verify that `from_byte()` (the replacement for the old `funty()` method)
// works correctly via `LinkReference` from `platform-num`.
//
// The old `funty()` method on `LinkType` was replaced by `from_byte()` on
// `LinkReference` as part of the trait unification (issue #30).

#[cfg(test)]
mod tests {
    use platform_num::LinkReference;
    use platform_trees::LinkType;

    #[test]
    fn test_from_byte_zero_all_types() {
        assert_eq!(u8::from_byte(0), 0u8);
        assert_eq!(u16::from_byte(0), 0u16);
        assert_eq!(u32::from_byte(0), 0u32);
        assert_eq!(u64::from_byte(0), 0u64);
        assert_eq!(u128::from_byte(0), 0u128);
        assert_eq!(usize::from_byte(0), 0usize);
    }

    #[test]
    fn test_from_byte_one_all_types() {
        assert_eq!(u8::from_byte(1), 1u8);
        assert_eq!(u16::from_byte(1), 1u16);
        assert_eq!(u32::from_byte(1), 1u32);
        assert_eq!(u64::from_byte(1), 1u64);
        assert_eq!(u128::from_byte(1), 1u128);
        assert_eq!(usize::from_byte(1), 1usize);
    }

    #[test]
    fn test_from_byte_two_all_types() {
        assert_eq!(u8::from_byte(2), 2u8);
        assert_eq!(u16::from_byte(2), 2u16);
        assert_eq!(u32::from_byte(2), 2u32);
        assert_eq!(u64::from_byte(2), 2u64);
        assert_eq!(u128::from_byte(2), 2u128);
        assert_eq!(usize::from_byte(2), 2usize);
    }

    fn generic_from_byte_usage<T: LinkReference>() -> (T, T, T) {
        (T::from_byte(0), T::from_byte(1), T::from_byte(2))
    }

    #[test]
    fn test_generic_from_byte_usage() {
        let (zero, one, two) = generic_from_byte_usage::<u32>();
        assert_eq!(zero, 0u32);
        assert_eq!(one, 1u32);
        assert_eq!(two, 2u32);

        let (zero, one, two) = generic_from_byte_usage::<u64>();
        assert_eq!(zero, 0u64);
        assert_eq!(one, 1u64);
        assert_eq!(two, 2u64);

        let (zero, one, two) = generic_from_byte_usage::<usize>();
        assert_eq!(zero, 0usize);
        assert_eq!(one, 1usize);
        assert_eq!(two, 2usize);
    }

    #[test]
    fn test_from_byte_comparison_patterns() {
        fn check_null<T: LinkReference>(value: T) -> bool {
            value == T::from_byte(0)
        }

        assert!(check_null(0u32));
        assert!(!check_null(1u32));
        assert!(check_null(0u64));
        assert!(!check_null(42u64));
    }

    #[test]
    fn test_from_byte_arithmetic_patterns() {
        fn inc_value<T: LinkReference>(val: T) -> T {
            val + T::from_byte(1)
        }

        fn dec_value<T: LinkReference>(val: T) -> T {
            val - T::from_byte(1)
        }

        assert_eq!(inc_value(5u32), 6u32);
        assert_eq!(dec_value(5u32), 4u32);
        assert_eq!(inc_value(0u64), 1u64);
        assert_eq!(dec_value(10u64), 9u64);
    }

    #[test]
    fn test_link_type_is_link_reference_superset() {
        fn accepts_link_type<T: LinkType>(_val: T) {}
        fn accepts_link_reference<T: LinkReference>(_val: T) {}

        accepts_link_type(42u32);
        accepts_link_reference(42u32);
        accepts_link_type(42u64);
        accepts_link_reference(42u64);
        accepts_link_type(42usize);
        accepts_link_reference(42usize);
        accepts_link_type(42u128);
        accepts_link_reference(42u128);
    }
}

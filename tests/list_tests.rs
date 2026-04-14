mod common;

use common::{TestAbsoluteList, TestRelativeList};
use platform_trees::{
    AbsoluteCircularLinkedList, AbsoluteLinkedList, LinkedList, RelativeCircularLinkedList,
    RelativeLinkedList,
};

// =============================================================================
// LinkReference trait tests
// =============================================================================

mod link_reference_tests {
    use platform_num::LinkReference;

    #[test]
    fn test_from_byte_zero() {
        assert_eq!(usize::from_byte(0), 0usize);
        assert_eq!(u8::from_byte(0), 0u8);
        assert_eq!(u16::from_byte(0), 0u16);
        assert_eq!(u32::from_byte(0), 0u32);
        assert_eq!(u64::from_byte(0), 0u64);
    }

    #[test]
    fn test_from_byte_one() {
        assert_eq!(usize::from_byte(1), 1usize);
        assert_eq!(u8::from_byte(1), 1u8);
        assert_eq!(u16::from_byte(1), 1u16);
        assert_eq!(u32::from_byte(1), 1u32);
        assert_eq!(u64::from_byte(1), 1u64);
    }

    #[test]
    fn test_from_byte_various_values() {
        for i in 0..=255u8 {
            assert_eq!(u8::from_byte(i), i);
            assert_eq!(u16::from_byte(i), u16::from(i));
            assert_eq!(u32::from_byte(i), u32::from(i));
            assert_eq!(u64::from_byte(i), u64::from(i));
            assert_eq!(usize::from_byte(i), i as usize);
        }
    }
}

// =============================================================================
// LinkedList trait tests
// =============================================================================

mod linked_list_tests {
    use super::*;

    #[test]
    fn test_get_set_previous() {
        let mut list = TestAbsoluteList::new(10);
        list.set_previous(1, 5);
        assert_eq!(list.get_previous(1), 5);
    }

    #[test]
    fn test_get_set_next() {
        let mut list = TestAbsoluteList::new(10);
        list.set_next(1, 3);
        assert_eq!(list.get_next(1), 3);
    }
}

// =============================================================================
// AbsoluteLinkedList trait tests
// =============================================================================

mod absolute_linked_list_tests {
    use super::*;

    #[test]
    fn test_get_set_first() {
        let mut list = TestAbsoluteList::new(10);
        list.set_first(5);
        assert_eq!(list.get_first(), 5);
    }

    #[test]
    fn test_get_set_last() {
        let mut list = TestAbsoluteList::new(10);
        list.set_last(7);
        assert_eq!(list.get_last(), 7);
    }

    #[test]
    fn test_get_set_size() {
        let mut list = TestAbsoluteList::new(10);
        list.set_size(3);
        assert_eq!(list.get_size(), 3);
    }

    #[test]
    fn test_inc_size() {
        let mut list = TestAbsoluteList::new(10);
        list.set_size(5);
        list.inc_size();
        assert_eq!(list.get_size(), 6);
    }

    #[test]
    fn test_dec_size() {
        let mut list = TestAbsoluteList::new(10);
        list.set_size(5);
        list.dec_size();
        assert_eq!(list.get_size(), 4);
    }
}

// =============================================================================
// AbsoluteCircularLinkedList trait tests
// =============================================================================

mod absolute_circular_linked_list_tests {
    use super::*;

    #[test]
    fn test_attach_as_first_empty_list() {
        let mut list = TestAbsoluteList::new(10);

        list.attach_as_first(1);

        assert_eq!(list.get_first(), 1);
        assert_eq!(list.get_last(), 1);
        assert_eq!(list.get_size(), 1);
        assert_eq!(list.get_previous(1), 1); // Points to itself
        assert_eq!(list.get_next(1), 1); // Points to itself
    }

    #[test]
    fn test_attach_as_first_non_empty_list() {
        let mut list = TestAbsoluteList::new(10);

        list.attach_as_first(1);
        list.attach_as_first(2);

        assert_eq!(list.get_first(), 2);
        assert_eq!(list.get_last(), 1);
        assert_eq!(list.get_size(), 2);
    }

    #[test]
    fn test_attach_as_last_empty_list() {
        let mut list = TestAbsoluteList::new(10);

        list.attach_as_last(1);

        assert_eq!(list.get_first(), 1);
        assert_eq!(list.get_last(), 1);
        assert_eq!(list.get_size(), 1);
    }

    #[test]
    fn test_attach_as_last_non_empty_list() {
        let mut list = TestAbsoluteList::new(10);

        list.attach_as_last(1);
        list.attach_as_last(2);

        assert_eq!(list.get_first(), 1);
        assert_eq!(list.get_last(), 2);
        assert_eq!(list.get_size(), 2);
    }

    #[test]
    fn test_attach_before() {
        let mut list = TestAbsoluteList::new(10);

        // Create list: 1 <-> 2
        list.attach_as_first(1);
        list.attach_as_last(2);

        // Insert 3 before 2: 1 <-> 3 <-> 2
        list.attach_before(2, 3);

        assert_eq!(list.get_first(), 1);
        assert_eq!(list.get_last(), 2);
        assert_eq!(list.get_size(), 3);
        assert_eq!(list.get_next(1), 3);
        assert_eq!(list.get_next(3), 2);
        assert_eq!(list.get_previous(2), 3);
    }

    #[test]
    fn test_attach_before_first() {
        let mut list = TestAbsoluteList::new(10);

        list.attach_as_first(1);
        list.attach_before(1, 2);

        assert_eq!(list.get_first(), 2);
        assert_eq!(list.get_last(), 1);
        assert_eq!(list.get_size(), 2);
    }

    #[test]
    fn test_attach_after() {
        let mut list = TestAbsoluteList::new(10);

        // Create list: 1 <-> 2
        list.attach_as_first(1);
        list.attach_as_last(2);

        // Insert 3 after 1: 1 <-> 3 <-> 2
        list.attach_after(1, 3);

        assert_eq!(list.get_first(), 1);
        assert_eq!(list.get_last(), 2);
        assert_eq!(list.get_size(), 3);
        assert_eq!(list.get_next(1), 3);
        assert_eq!(list.get_next(3), 2);
    }

    #[test]
    fn test_attach_after_last() {
        let mut list = TestAbsoluteList::new(10);

        list.attach_as_first(1);
        list.attach_after(1, 2);

        assert_eq!(list.get_first(), 1);
        assert_eq!(list.get_last(), 2);
        assert_eq!(list.get_size(), 2);
    }

    #[test]
    fn test_detach_single_element() {
        let mut list = TestAbsoluteList::new(10);

        list.attach_as_first(1);
        list.detach(1);

        assert_eq!(list.get_first(), 0);
        assert_eq!(list.get_last(), 0);
        assert_eq!(list.get_size(), 0);
        assert_eq!(list.get_previous(1), 0);
        assert_eq!(list.get_next(1), 0);
    }

    #[test]
    fn test_detach_first_element() {
        let mut list = TestAbsoluteList::new(10);

        list.attach_as_first(1);
        list.attach_as_last(2);
        list.attach_as_last(3);

        list.detach(1);

        assert_eq!(list.get_first(), 2);
        assert_eq!(list.get_last(), 3);
        assert_eq!(list.get_size(), 2);
    }

    #[test]
    fn test_detach_last_element() {
        let mut list = TestAbsoluteList::new(10);

        list.attach_as_first(1);
        list.attach_as_last(2);
        list.attach_as_last(3);

        list.detach(3);

        assert_eq!(list.get_first(), 1);
        assert_eq!(list.get_last(), 2);
        assert_eq!(list.get_size(), 2);
    }

    #[test]
    fn test_detach_middle_element() {
        let mut list = TestAbsoluteList::new(10);

        list.attach_as_first(1);
        list.attach_as_last(2);
        list.attach_as_last(3);

        list.detach(2);

        assert_eq!(list.get_first(), 1);
        assert_eq!(list.get_last(), 3);
        assert_eq!(list.get_size(), 2);
        assert_eq!(list.get_next(1), 3);
        assert_eq!(list.get_previous(3), 1);
    }

    #[test]
    fn test_circular_structure() {
        let mut list = TestAbsoluteList::new(10);

        list.attach_as_first(1);
        list.attach_as_last(2);
        list.attach_as_last(3);

        // Check circular structure
        assert_eq!(list.get_next(3), 1); // Last points to first
        assert_eq!(list.get_previous(1), 3); // First points back to last
    }
}

// =============================================================================
// RelativeLinkedList trait tests
// =============================================================================

mod relative_linked_list_tests {
    use super::*;

    #[test]
    fn test_get_set_first() {
        let mut list = TestRelativeList::new(10);
        list.set_first(1, 5);
        assert_eq!(list.get_first(1), 5);
    }

    #[test]
    fn test_get_set_last() {
        let mut list = TestRelativeList::new(10);
        list.set_last(1, 7);
        assert_eq!(list.get_last(1), 7);
    }

    #[test]
    fn test_get_set_size() {
        let mut list = TestRelativeList::new(10);
        list.set_size(1, 3);
        assert_eq!(list.get_size(1), 3);
    }

    #[test]
    fn test_inc_size() {
        let mut list = TestRelativeList::new(10);
        list.set_size(1, 5);
        list.inc_size(1);
        assert_eq!(list.get_size(1), 6);
    }

    #[test]
    fn test_dec_size() {
        let mut list = TestRelativeList::new(10);
        list.set_size(1, 5);
        list.dec_size(1);
        assert_eq!(list.get_size(1), 4);
    }

    #[test]
    fn test_multiple_heads() {
        let mut list = TestRelativeList::new(10);

        list.set_first(1, 2);
        list.set_first(2, 3);

        assert_eq!(list.get_first(1), 2);
        assert_eq!(list.get_first(2), 3);
    }
}

// =============================================================================
// RelativeCircularLinkedList trait tests
// =============================================================================

mod relative_circular_linked_list_tests {
    use super::*;

    const HEAD: usize = 1;

    #[test]
    fn test_attach_as_first_empty_list() {
        let mut list = TestRelativeList::new(10);

        list.attach_as_first(HEAD, 2);

        assert_eq!(list.get_first(HEAD), 2);
        assert_eq!(list.get_last(HEAD), 2);
        assert_eq!(list.get_size(HEAD), 1);
        assert_eq!(list.get_previous(2), 2);
        assert_eq!(list.get_next(2), 2);
    }

    #[test]
    fn test_attach_as_first_non_empty_list() {
        let mut list = TestRelativeList::new(10);

        list.attach_as_first(HEAD, 2);
        list.attach_as_first(HEAD, 3);

        assert_eq!(list.get_first(HEAD), 3);
        assert_eq!(list.get_last(HEAD), 2);
        assert_eq!(list.get_size(HEAD), 2);
    }

    #[test]
    fn test_attach_as_last_empty_list() {
        let mut list = TestRelativeList::new(10);

        list.attach_as_last(HEAD, 2);

        assert_eq!(list.get_first(HEAD), 2);
        assert_eq!(list.get_last(HEAD), 2);
        assert_eq!(list.get_size(HEAD), 1);
    }

    #[test]
    fn test_attach_as_last_non_empty_list() {
        let mut list = TestRelativeList::new(10);

        list.attach_as_last(HEAD, 2);
        list.attach_as_last(HEAD, 3);

        assert_eq!(list.get_first(HEAD), 2);
        assert_eq!(list.get_last(HEAD), 3);
        assert_eq!(list.get_size(HEAD), 2);
    }

    #[test]
    fn test_attach_before() {
        let mut list = TestRelativeList::new(10);

        list.attach_as_first(HEAD, 2);
        list.attach_as_last(HEAD, 3);
        list.attach_before(HEAD, 3, 4);

        assert_eq!(list.get_first(HEAD), 2);
        assert_eq!(list.get_last(HEAD), 3);
        assert_eq!(list.get_size(HEAD), 3);
        assert_eq!(list.get_next(2), 4);
        assert_eq!(list.get_next(4), 3);
    }

    #[test]
    fn test_attach_before_first() {
        let mut list = TestRelativeList::new(10);

        list.attach_as_first(HEAD, 2);
        list.attach_before(HEAD, 2, 3);

        assert_eq!(list.get_first(HEAD), 3);
        assert_eq!(list.get_last(HEAD), 2);
        assert_eq!(list.get_size(HEAD), 2);
    }

    #[test]
    fn test_attach_after() {
        let mut list = TestRelativeList::new(10);

        list.attach_as_first(HEAD, 2);
        list.attach_as_last(HEAD, 3);
        list.attach_after(HEAD, 2, 4);

        assert_eq!(list.get_first(HEAD), 2);
        assert_eq!(list.get_last(HEAD), 3);
        assert_eq!(list.get_size(HEAD), 3);
        assert_eq!(list.get_next(2), 4);
        assert_eq!(list.get_next(4), 3);
    }

    #[test]
    fn test_attach_after_last() {
        let mut list = TestRelativeList::new(10);

        list.attach_as_first(HEAD, 2);
        list.attach_after(HEAD, 2, 3);

        assert_eq!(list.get_first(HEAD), 2);
        assert_eq!(list.get_last(HEAD), 3);
        assert_eq!(list.get_size(HEAD), 2);
    }

    #[test]
    fn test_detach_single_element() {
        let mut list = TestRelativeList::new(10);

        list.attach_as_first(HEAD, 2);
        list.detach(HEAD, 2);

        assert_eq!(list.get_first(HEAD), 0);
        assert_eq!(list.get_last(HEAD), 0);
        assert_eq!(list.get_size(HEAD), 0);
        assert_eq!(list.get_previous(2), 0);
        assert_eq!(list.get_next(2), 0);
    }

    #[test]
    fn test_detach_first_element() {
        let mut list = TestRelativeList::new(10);

        list.attach_as_first(HEAD, 2);
        list.attach_as_last(HEAD, 3);
        list.attach_as_last(HEAD, 4);

        list.detach(HEAD, 2);

        assert_eq!(list.get_first(HEAD), 3);
        assert_eq!(list.get_last(HEAD), 4);
        assert_eq!(list.get_size(HEAD), 2);
    }

    #[test]
    fn test_detach_last_element() {
        let mut list = TestRelativeList::new(10);

        list.attach_as_first(HEAD, 2);
        list.attach_as_last(HEAD, 3);
        list.attach_as_last(HEAD, 4);

        list.detach(HEAD, 4);

        assert_eq!(list.get_first(HEAD), 2);
        assert_eq!(list.get_last(HEAD), 3);
        assert_eq!(list.get_size(HEAD), 2);
    }

    #[test]
    fn test_detach_middle_element() {
        let mut list = TestRelativeList::new(10);

        list.attach_as_first(HEAD, 2);
        list.attach_as_last(HEAD, 3);
        list.attach_as_last(HEAD, 4);

        list.detach(HEAD, 3);

        assert_eq!(list.get_first(HEAD), 2);
        assert_eq!(list.get_last(HEAD), 4);
        assert_eq!(list.get_size(HEAD), 2);
        assert_eq!(list.get_next(2), 4);
        assert_eq!(list.get_previous(4), 2);
    }

    #[test]
    fn test_multiple_lists() {
        const HEAD1: usize = 1;
        const HEAD2: usize = 10;
        let mut list = TestRelativeList::new(20);

        list.attach_as_first(HEAD1, 2);
        list.attach_as_last(HEAD1, 3);

        list.attach_as_first(HEAD2, 11);
        list.attach_as_last(HEAD2, 12);

        assert_eq!(list.get_first(HEAD1), 2);
        assert_eq!(list.get_last(HEAD1), 3);
        assert_eq!(list.get_size(HEAD1), 2);

        assert_eq!(list.get_first(HEAD2), 11);
        assert_eq!(list.get_last(HEAD2), 12);
        assert_eq!(list.get_size(HEAD2), 2);
    }
}

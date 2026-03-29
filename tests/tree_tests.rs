mod common;

use common::TestTree;
use platform_trees::RecursiveSizeBalancedTree;

// =============================================================================
// SizeBalancedTree trait tests (RecursiveSizeBalancedTree)
// =============================================================================

mod size_balanced_tree_tests {
    use super::*;

    #[test]
    fn test_get_set_left() {
        let mut tree = TestTree::new(10);
        unsafe {
            tree.set_left(1, 2);
            assert_eq!(tree.get_left(1), 2);
        }
    }

    #[test]
    fn test_get_set_right() {
        let mut tree = TestTree::new(10);
        unsafe {
            tree.set_right(1, 3);
            assert_eq!(tree.get_right(1), 3);
        }
    }

    #[test]
    fn test_get_set_size() {
        let mut tree = TestTree::new(10);
        unsafe {
            tree.set_size(1, 5);
            assert_eq!(tree.get_size(1), 5);
        }
    }

    #[test]
    fn test_get_left_or_default_with_zero() {
        let tree = TestTree::new(10);
        unsafe {
            assert_eq!(tree.get_left_or_default(0), 0);
        }
    }

    #[test]
    fn test_get_left_or_default_with_node() {
        let mut tree = TestTree::new(10);
        unsafe {
            tree.set_left(1, 2);
            assert_eq!(tree.get_left_or_default(1), 2);
        }
    }

    #[test]
    fn test_get_right_or_default_with_zero() {
        let tree = TestTree::new(10);
        unsafe {
            assert_eq!(tree.get_right_or_default(0), 0);
        }
    }

    #[test]
    fn test_get_right_or_default_with_node() {
        let mut tree = TestTree::new(10);
        unsafe {
            tree.set_right(1, 3);
            assert_eq!(tree.get_right_or_default(1), 3);
        }
    }

    #[test]
    fn test_get_size_or_zero_with_zero() {
        let tree = TestTree::new(10);
        unsafe {
            assert_eq!(tree.get_size_or_zero(0), 0);
        }
    }

    #[test]
    fn test_get_size_or_zero_with_node() {
        let mut tree = TestTree::new(10);
        unsafe {
            tree.set_size(1, 5);
            assert_eq!(tree.get_size_or_zero(1), 5);
        }
    }

    #[test]
    fn test_inc_size() {
        let mut tree = TestTree::new(10);
        unsafe {
            tree.set_size(1, 5);
            tree.inc_size(1);
            assert_eq!(tree.get_size(1), 6);
        }
    }

    #[test]
    fn test_dec_size() {
        let mut tree = TestTree::new(10);
        unsafe {
            tree.set_size(1, 5);
            tree.dec_size(1);
            assert_eq!(tree.get_size(1), 4);
        }
    }

    #[test]
    fn test_get_left_size() {
        let mut tree = TestTree::new(10);
        unsafe {
            tree.set_left(1, 2);
            tree.set_size(2, 3);
            assert_eq!(tree.get_left_size(1), 3);
        }
    }

    #[test]
    fn test_get_right_size() {
        let mut tree = TestTree::new(10);
        unsafe {
            tree.set_right(1, 3);
            tree.set_size(3, 5);
            assert_eq!(tree.get_right_size(1), 5);
        }
    }

    #[test]
    fn test_fix_size() {
        let mut tree = TestTree::new(10);
        unsafe {
            // Create tree: 1 -> (2, 3)
            tree.set_left(1, 2);
            tree.set_right(1, 3);
            tree.set_size(2, 1);
            tree.set_size(3, 2);

            tree.fix_size(1);
            assert_eq!(tree.get_size(1), 4); // 1 + 2 + 1
        }
    }

    #[test]
    fn test_left_rotate_core() {
        let mut tree = TestTree::new(10);
        unsafe {
            // Initial: 1 -> (_, 2) where 2 -> (3, _)
            tree.set_right(1, 2);
            tree.set_left(2, 3);
            tree.set_size(1, 3);
            tree.set_size(2, 2);
            tree.set_size(3, 1);

            let new_root = tree.left_rotate_core(1);

            assert_eq!(new_root, 2);
            assert_eq!(tree.get_left(2), 1);
            assert_eq!(tree.get_right(1), 3);
            assert_eq!(tree.get_size(2), 3);
        }
    }

    #[test]
    fn test_left_rotate() {
        let mut tree = TestTree::new(10);
        unsafe {
            tree.set_right(1, 2);
            tree.set_left(2, 3);
            tree.set_size(1, 3);
            tree.set_size(2, 2);
            tree.set_size(3, 1);

            let mut root: usize = 1;
            tree.left_rotate(&mut root);

            assert_eq!(root, 2);
        }
    }

    #[test]
    fn test_right_rotate_core() {
        let mut tree = TestTree::new(10);
        unsafe {
            // Initial: 1 -> (2, _) where 2 -> (_, 3)
            tree.set_left(1, 2);
            tree.set_right(2, 3);
            tree.set_size(1, 3);
            tree.set_size(2, 2);
            tree.set_size(3, 1);

            let new_root = tree.right_rotate_core(1);

            assert_eq!(new_root, 2);
            assert_eq!(tree.get_right(2), 1);
            assert_eq!(tree.get_left(1), 3);
            assert_eq!(tree.get_size(2), 3);
        }
    }

    #[test]
    fn test_right_rotate() {
        let mut tree = TestTree::new(10);
        unsafe {
            tree.set_left(1, 2);
            tree.set_right(2, 3);
            tree.set_size(1, 3);
            tree.set_size(2, 2);
            tree.set_size(3, 1);

            let mut root: usize = 1;
            tree.right_rotate(&mut root);

            assert_eq!(root, 2);
        }
    }

    #[test]
    fn test_get_rightest() {
        let mut tree = TestTree::new(10);
        unsafe {
            // Create tree: 1 -> (_, 2) where 2 -> (_, 3)
            tree.set_right(1, 2);
            tree.set_right(2, 3);

            assert_eq!(tree.get_rightest(1), 3);
        }
    }

    #[test]
    fn test_get_leftest() {
        let mut tree = TestTree::new(10);
        unsafe {
            // Create tree: 3 -> (2, _) where 2 -> (1, _)
            tree.set_left(3, 2);
            tree.set_left(2, 1);

            assert_eq!(tree.get_leftest(3), 1);
        }
    }

    #[test]
    fn test_get_next() {
        let mut tree = TestTree::new(10);
        unsafe {
            // Node 1 has right child 2 which has left child 3
            tree.set_right(1, 2);
            tree.set_left(2, 3);

            assert_eq!(tree.get_next(1), 3);
        }
    }

    #[test]
    fn test_get_previous() {
        let mut tree = TestTree::new(10);
        unsafe {
            // Node 3 has left child 2 which has right child 1
            tree.set_left(3, 2);
            tree.set_right(2, 1);

            assert_eq!(tree.get_previous(3), 1);
        }
    }

    #[test]
    fn test_contains_found() {
        let mut tree = TestTree::new(10);
        unsafe {
            // Create BST: 5 -> (3, 7)
            tree.set_left(5, 3);
            tree.set_right(5, 7);

            assert!(tree.contains(5, 5));
            assert!(tree.contains(3, 5));
            assert!(tree.contains(7, 5));
        }
    }

    #[test]
    fn test_contains_not_found() {
        let mut tree = TestTree::new(10);
        unsafe {
            // Create BST: 5 -> (3, 7)
            tree.set_left(5, 3);
            tree.set_right(5, 7);

            assert!(!tree.contains(2, 5));
            assert!(!tree.contains(4, 5));
            assert!(!tree.contains(8, 5));
        }
    }

    #[test]
    fn test_contains_empty_tree() {
        let tree = TestTree::new(10);
        unsafe {
            assert!(!tree.contains(1, 0));
        }
    }

    #[test]
    fn test_clear_node() {
        let mut tree = TestTree::new(10);
        unsafe {
            tree.set_left(1, 2);
            tree.set_right(1, 3);
            tree.set_size(1, 5);

            tree.clear_node(1);

            assert_eq!(tree.get_left(1), 0);
            assert_eq!(tree.get_right(1), 0);
            assert_eq!(tree.get_size(1), 0);
        }
    }

    #[test]
    fn test_first_is_to_the_left_of_second() {
        let tree = TestTree::new(10);
        unsafe {
            assert!(tree.first_is_to_the_left_of_second(1, 2));
            assert!(!tree.first_is_to_the_left_of_second(2, 1));
            assert!(!tree.first_is_to_the_left_of_second(1, 1));
        }
    }

    #[test]
    fn test_first_is_to_the_right_of_second() {
        let tree = TestTree::new(10);
        unsafe {
            assert!(tree.first_is_to_the_right_of_second(2, 1));
            assert!(!tree.first_is_to_the_right_of_second(1, 2));
            assert!(!tree.first_is_to_the_right_of_second(1, 1));
        }
    }

    #[test]
    fn test_get_mut_left_reference() {
        let mut tree = TestTree::new(10);
        unsafe {
            let ptr = tree.get_mut_left_reference(1);
            *ptr = 5;
            assert_eq!(tree.get_left(1), 5);
        }
    }

    #[test]
    fn test_get_mut_right_reference() {
        let mut tree = TestTree::new(10);
        unsafe {
            let ptr = tree.get_mut_right_reference(1);
            *ptr = 7;
            assert_eq!(tree.get_right(1), 7);
        }
    }

    #[test]
    fn test_get_left_reference() {
        let mut tree = TestTree::new(10);
        unsafe {
            tree.set_left(1, 3);
            let ptr = tree.get_left_reference(1);
            assert_eq!(*ptr, 3);
        }
    }

    #[test]
    fn test_get_right_reference() {
        let mut tree = TestTree::new(10);
        unsafe {
            tree.set_right(1, 4);
            let ptr = tree.get_right_reference(1);
            assert_eq!(*ptr, 4);
        }
    }
}

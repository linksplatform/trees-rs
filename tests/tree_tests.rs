mod common;

use common::TestTree;
use platform_trees::{IterativeSizeBalancedTree, RecursiveSizeBalancedTree};

// =============================================================================
// SizeBalancedTree trait tests
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

// =============================================================================
// IterativeSizeBalancedTree trait tests
// =============================================================================

mod iterative_size_balanced_tree_tests {
    use super::*;

    #[test]
    fn test_attach_to_empty_tree() {
        let mut tree = TestTree::new(10);
        let mut root: usize = 0;

        unsafe {
            tree.attach(&mut root, 5);
        }

        assert_eq!(root, 5);
        unsafe {
            assert_eq!(tree.get_size(5), 1);
        }
    }

    #[test]
    fn test_attach_single_node_to_left() {
        let mut tree = TestTree::new(10);
        let mut root: usize = 0;

        unsafe {
            tree.attach(&mut root, 5);
            tree.attach(&mut root, 3);
        }

        unsafe {
            assert_eq!(tree.get_left(5), 3);
            assert_eq!(tree.get_size(5), 2);
            assert_eq!(tree.get_size(3), 1);
        }
    }

    #[test]
    fn test_attach_single_node_to_right() {
        let mut tree = TestTree::new(10);
        let mut root: usize = 0;

        unsafe {
            tree.attach(&mut root, 5);
            tree.attach(&mut root, 7);
        }

        unsafe {
            assert_eq!(tree.get_right(5), 7);
            assert_eq!(tree.get_size(5), 2);
            assert_eq!(tree.get_size(7), 1);
        }
    }

    #[test]
    fn test_attach_multiple_nodes() {
        let mut tree = TestTree::new(20);
        let mut root: usize = 0;

        unsafe {
            tree.attach(&mut root, 10);
            tree.attach(&mut root, 5);
            tree.attach(&mut root, 15);
            tree.attach(&mut root, 3);
            tree.attach(&mut root, 7);
        }

        unsafe {
            assert!(tree.contains(10, root));
            assert!(tree.contains(5, root));
            assert!(tree.contains(15, root));
            assert!(tree.contains(3, root));
            assert!(tree.contains(7, root));
            assert_eq!(tree.get_size(root), 5);
        }
    }

    #[test]
    fn test_detach_from_single_node_tree() {
        let mut tree = TestTree::new(10);
        let mut root: usize = 0;

        unsafe {
            tree.attach(&mut root, 5);
            tree.detach(&mut root, 5);
        }

        assert_eq!(root, 0);
    }

    #[test]
    fn test_detach_leaf_node() {
        let mut tree = TestTree::new(20);
        let mut root: usize = 0;

        unsafe {
            tree.attach(&mut root, 10);
            tree.attach(&mut root, 5);
            tree.attach(&mut root, 15);

            tree.detach(&mut root, 5);

            assert!(!tree.contains(5, root));
            assert!(tree.contains(10, root));
            assert!(tree.contains(15, root));
        }
    }

    #[test]
    fn test_detach_node_with_one_child() {
        let mut tree = TestTree::new(20);
        let mut root: usize = 0;

        unsafe {
            tree.attach(&mut root, 10);
            tree.attach(&mut root, 5);
            tree.attach(&mut root, 3);

            tree.detach(&mut root, 5);

            assert!(!tree.contains(5, root));
            assert!(tree.contains(10, root));
            assert!(tree.contains(3, root));
        }
    }

    #[test]
    fn test_detach_node_with_two_children() {
        let mut tree = TestTree::new(20);
        let mut root: usize = 0;

        unsafe {
            tree.attach(&mut root, 10);
            tree.attach(&mut root, 5);
            tree.attach(&mut root, 15);
            tree.attach(&mut root, 3);
            tree.attach(&mut root, 7);

            tree.detach(&mut root, 5);

            assert!(!tree.contains(5, root));
            assert!(tree.contains(10, root));
            assert!(tree.contains(15, root));
            assert!(tree.contains(3, root));
            assert!(tree.contains(7, root));
        }
    }

    #[test]
    fn test_detach_root() {
        let mut tree = TestTree::new(20);
        let mut root: usize = 0;

        unsafe {
            tree.attach(&mut root, 10);
            tree.attach(&mut root, 5);
            tree.attach(&mut root, 15);

            tree.detach(&mut root, 10);

            assert!(!tree.contains(10, root));
            assert!(tree.contains(5, root));
            assert!(tree.contains(15, root));
            assert_ne!(root, 0);
        }
    }

    #[test]
    fn test_attach_and_detach_sequence() {
        let mut tree = TestTree::new(30);
        let mut root: usize = 0;

        unsafe {
            // Build tree
            for i in [10, 5, 15, 3, 7, 12, 17] {
                tree.attach(&mut root, i);
            }

            // Verify all nodes present
            for i in [10, 5, 15, 3, 7, 12, 17] {
                assert!(tree.contains(i, root), "Node {i} should be present");
            }

            // Remove some nodes
            tree.detach(&mut root, 3);
            tree.detach(&mut root, 17);
            tree.detach(&mut root, 10);

            // Verify correct nodes present
            assert!(!tree.contains(3, root));
            assert!(!tree.contains(17, root));
            assert!(!tree.contains(10, root));
            assert!(tree.contains(5, root));
            assert!(tree.contains(15, root));
            assert!(tree.contains(7, root));
            assert!(tree.contains(12, root));
        }
    }

    #[test]
    fn test_tree_balancing() {
        let mut tree = TestTree::new(20);
        let mut root: usize = 0;

        unsafe {
            // Insert in order that would cause imbalance without rotations
            for i in 1..=10 {
                tree.attach(&mut root, i);
            }

            // All nodes should be reachable
            for i in 1..=10 {
                assert!(tree.contains(i, root), "Node {i} should be present");
            }

            assert_eq!(tree.get_size(root), 10);
        }
    }

    #[test]
    fn test_detach_all_nodes() {
        let mut tree = TestTree::new(20);
        let mut root: usize = 0;

        unsafe {
            for i in 1..=5 {
                tree.attach(&mut root, i);
            }

            for i in 1..=5 {
                tree.detach(&mut root, i);
            }

            assert_eq!(root, 0);
        }
    }

    // Additional edge case tests for full coverage

    #[test]
    fn test_attach_left_right_rotation_edge_case() {
        // Tests the path where we need a left-right double rotation on left side
        let mut tree = TestTree::new(30);
        let mut root: usize = 0;

        unsafe {
            // Build a tree that will trigger left-right rotation
            // Start with 10, then 5, then insert 8 (between 5 and 10)
            tree.attach(&mut root, 10);
            tree.attach(&mut root, 5);
            tree.attach(&mut root, 8); // This goes right of 5, should trigger LR rotation

            assert!(tree.contains(10, root));
            assert!(tree.contains(5, root));
            assert!(tree.contains(8, root));
            assert_eq!(tree.get_size(root), 3);
        }
    }

    #[test]
    fn test_attach_descend_left_without_rotation() {
        // Tests the case where we descend left without rotation
        let mut tree = TestTree::new(40);
        let mut root: usize = 0;

        unsafe {
            // Build a more balanced tree first
            tree.attach(&mut root, 20);
            tree.attach(&mut root, 10);
            tree.attach(&mut root, 30);
            tree.attach(&mut root, 25);
            tree.attach(&mut root, 35);
            // Now insert something that needs to go left but doesn't trigger rotation
            tree.attach(&mut root, 5);
            tree.attach(&mut root, 2);

            assert!(tree.contains(2, root));
            assert_eq!(tree.get_size(root), 7);
        }
    }

    #[test]
    fn test_attach_special_case_left_right_empty() {
        // Tests the special case in attach_core where left_right_size == 0 and right_size == 0
        let mut tree = TestTree::new(20);
        let mut root: usize = 0;

        unsafe {
            // Create specific tree structure
            tree.attach(&mut root, 10);
            tree.attach(&mut root, 5);
            // Now insert 7 which is right of 5 but left of 10
            // With just 2 nodes, this triggers the special case
            tree.attach(&mut root, 7);

            assert!(tree.contains(10, root));
            assert!(tree.contains(5, root));
            assert!(tree.contains(7, root));
        }
    }

    #[test]
    fn test_attach_right_left_rotation_edge_case() {
        // Tests the path where we need a right-left double rotation on right side
        let mut tree = TestTree::new(30);
        let mut root: usize = 0;

        unsafe {
            // Build a tree that will trigger right-left rotation
            tree.attach(&mut root, 10);
            tree.attach(&mut root, 20);
            tree.attach(&mut root, 15); // Between 10 and 20, should trigger RL rotation

            assert!(tree.contains(10, root));
            assert!(tree.contains(15, root));
            assert!(tree.contains(20, root));
            assert_eq!(tree.get_size(root), 3);
        }
    }

    #[test]
    fn test_attach_descend_right_without_rotation() {
        // Tests the case where we descend right without rotation
        let mut tree = TestTree::new(40);
        let mut root: usize = 0;

        unsafe {
            // Build a balanced tree
            tree.attach(&mut root, 20);
            tree.attach(&mut root, 30);
            tree.attach(&mut root, 10);
            tree.attach(&mut root, 5);
            tree.attach(&mut root, 15);
            // Insert on the right side, descending
            tree.attach(&mut root, 35);
            tree.attach(&mut root, 40);

            assert!(tree.contains(40, root));
            assert_eq!(tree.get_size(root), 7);
        }
    }

    #[test]
    fn test_detach_with_double_rotation_left() {
        // Tests detach causing double rotation on left side
        let mut tree = TestTree::new(40);
        let mut root: usize = 0;

        unsafe {
            // Build larger tree
            for i in [20, 10, 30, 5, 15, 25, 35, 3, 8, 13, 17] {
                tree.attach(&mut root, i);
            }

            // Detach to trigger specific paths
            tree.detach(&mut root, 3);
            tree.detach(&mut root, 5);
            tree.detach(&mut root, 8);

            assert!(!tree.contains(3, root));
            assert!(!tree.contains(5, root));
            assert!(!tree.contains(8, root));
        }
    }

    #[test]
    fn test_detach_with_double_rotation_right() {
        // Tests detach causing double rotation on right side
        let mut tree = TestTree::new(50);
        let mut root: usize = 0;

        unsafe {
            // Build larger tree
            for i in [20, 10, 30, 5, 15, 25, 35, 32, 38, 28, 27] {
                tree.attach(&mut root, i);
            }

            // Detach to trigger specific paths
            tree.detach(&mut root, 35);
            tree.detach(&mut root, 38);
            tree.detach(&mut root, 32);

            assert!(!tree.contains(35, root));
            assert!(!tree.contains(38, root));
            assert!(!tree.contains(32, root));
        }
    }

    #[test]
    fn test_detach_with_left_only_child() {
        // Tests detach where replacement comes from left subtree
        let mut tree = TestTree::new(30);
        let mut root: usize = 0;

        unsafe {
            // Create tree where left subtree is larger
            tree.attach(&mut root, 10);
            tree.attach(&mut root, 5);
            tree.attach(&mut root, 15);
            tree.attach(&mut root, 3);
            tree.attach(&mut root, 7);
            tree.attach(&mut root, 2);
            tree.attach(&mut root, 4);

            // Detach a node that will use replacement from left (larger) subtree
            tree.detach(&mut root, 5);

            assert!(!tree.contains(5, root));
            assert!(tree.contains(3, root));
            assert!(tree.contains(7, root));
        }
    }

    #[test]
    fn test_detach_node_with_left_child_only() {
        // Tests the path where we set *root = *left
        let mut tree = TestTree::new(20);
        let mut root: usize = 0;

        unsafe {
            tree.attach(&mut root, 10);
            tree.attach(&mut root, 5);
            tree.attach(&mut root, 15);
            tree.attach(&mut root, 3);

            // Remove 5, which has only left child (3)
            tree.detach(&mut root, 5);

            assert!(!tree.contains(5, root));
            assert!(tree.contains(3, root));
        }
    }

    #[test]
    fn test_large_sequential_insert() {
        // Tests inserting many nodes in ascending order
        let mut tree = TestTree::new(50);
        let mut root: usize = 0;

        unsafe {
            for i in 1..=20 {
                tree.attach(&mut root, i);
            }

            for i in 1..=20 {
                assert!(tree.contains(i, root), "Node {i} missing");
            }
            assert_eq!(tree.get_size(root), 20);
        }
    }

    #[test]
    fn test_large_reverse_insert() {
        // Tests inserting many nodes in descending order
        let mut tree = TestTree::new(50);
        let mut root: usize = 0;

        unsafe {
            for i in (1..=20).rev() {
                tree.attach(&mut root, i);
            }

            for i in 1..=20 {
                assert!(tree.contains(i, root), "Node {i} missing");
            }
            assert_eq!(tree.get_size(root), 20);
        }
    }

    #[test]
    fn test_alternating_insert() {
        // Tests inserting nodes in alternating pattern
        let mut tree = TestTree::new(50);
        let mut root: usize = 0;

        unsafe {
            // Insert 10, 1, 20, 2, 19, 3, 18, etc.
            let mut low = 1;
            let mut high = 20;
            for i in 0..20 {
                if i % 2 == 0 {
                    tree.attach(&mut root, high);
                    high -= 1;
                } else {
                    tree.attach(&mut root, low);
                    low += 1;
                }
            }

            for i in 1..=20 {
                assert!(tree.contains(i, root), "Node {i} missing");
            }
        }
    }

    #[test]
    fn test_detach_from_large_tree() {
        // Tests various detach patterns on large tree
        let mut tree = TestTree::new(100);
        let mut root: usize = 0;

        unsafe {
            // Build tree with 50 nodes
            for i in 1..=50 {
                tree.attach(&mut root, i);
            }

            // Detach every other node
            for i in (1..=50).step_by(2) {
                tree.detach(&mut root, i);
            }

            // Check remaining nodes
            for i in (2..=50).step_by(2) {
                assert!(tree.contains(i, root), "Node {i} should be present");
            }
            for i in (1..=49).step_by(2) {
                assert!(!tree.contains(i, root), "Node {i} should be absent");
            }
        }
    }

    #[test]
    fn test_detach_node_with_right_child_only() {
        // Tests the path where we set *root = *right (right_size > 0, left_size == 0)
        let mut tree = TestTree::new(20);
        let mut root: usize = 0;

        unsafe {
            tree.attach(&mut root, 10);
            tree.attach(&mut root, 5);
            tree.attach(&mut root, 15);
            tree.attach(&mut root, 17);

            // Remove 15, which should have only right child (17) after tree balancing
            tree.detach(&mut root, 15);

            assert!(!tree.contains(15, root));
            assert!(tree.contains(17, root));
        }
    }

    #[test]
    fn test_attach_right_special_case_empty() {
        // Tests the special case in attach_core where right_left_size == 0 and left_size == 0
        let mut tree = TestTree::new(20);
        let mut root: usize = 0;

        unsafe {
            tree.attach(&mut root, 10);
            tree.attach(&mut root, 20);
            // Insert 15 which is left of 20 but right of 10
            // With just 2 nodes, this triggers the right-side special case
            tree.attach(&mut root, 15);

            assert!(tree.contains(10, root));
            assert!(tree.contains(15, root));
            assert!(tree.contains(20, root));
        }
    }

    #[test]
    fn test_detach_replacement_from_right_subtree() {
        // Tests detach where replacement comes from right subtree (left_size <= right_size)
        let mut tree = TestTree::new(30);
        let mut root: usize = 0;

        unsafe {
            tree.attach(&mut root, 10);
            tree.attach(&mut root, 5);
            tree.attach(&mut root, 15);
            tree.attach(&mut root, 12);
            tree.attach(&mut root, 17);
            tree.attach(&mut root, 16);
            tree.attach(&mut root, 18);

            // Detach 15 which has both children; right subtree is larger
            tree.detach(&mut root, 15);

            assert!(!tree.contains(15, root));
            assert!(tree.contains(12, root));
            assert!(tree.contains(17, root));
            assert!(tree.contains(16, root));
            assert!(tree.contains(18, root));
        }
    }

    #[test]
    fn test_get_rightest_single_node() {
        // Tests get_rightest with a single node (no right child)
        let mut tree = TestTree::new(10);
        unsafe {
            tree.set_size(1, 1);
            assert_eq!(tree.get_rightest(1), 1);
        }
    }

    #[test]
    fn test_get_leftest_single_node() {
        // Tests get_leftest with a single node (no left child)
        let mut tree = TestTree::new(10);
        unsafe {
            tree.set_size(1, 1);
            assert_eq!(tree.get_leftest(1), 1);
        }
    }

    #[test]
    fn test_get_left_size_no_left_child() {
        // Tests get_left_size when node has no left child
        let mut tree = TestTree::new(10);
        unsafe {
            tree.set_size(1, 1);
            assert_eq!(tree.get_left_size(1), 0);
        }
    }

    #[test]
    fn test_get_right_size_no_right_child() {
        // Tests get_right_size when node has no right child
        let mut tree = TestTree::new(10);
        unsafe {
            tree.set_size(1, 1);
            assert_eq!(tree.get_right_size(1), 0);
        }
    }

    #[test]
    fn test_fix_size_leaf_node() {
        // Tests fix_size on a leaf node (no children)
        let mut tree = TestTree::new(10);
        unsafe {
            tree.fix_size(1);
            assert_eq!(tree.get_size(1), 1); // 0 + 0 + 1
        }
    }

    #[test]
    fn test_detach_reinsert_cycle() {
        // Tests detaching all nodes and reinserting them
        let mut tree = TestTree::new(30);
        let mut root: usize = 0;

        unsafe {
            let nodes = [15, 10, 20, 5, 12, 17, 25];
            for &i in &nodes {
                tree.attach(&mut root, i);
            }

            // Detach all
            for &i in &nodes {
                tree.detach(&mut root, i);
            }
            assert_eq!(root, 0);

            // Reinsert all
            for &i in &nodes {
                tree.attach(&mut root, i);
            }

            for &i in &nodes {
                assert!(
                    tree.contains(i, root),
                    "Node {i} should be present after reinsert"
                );
            }
            assert_eq!(tree.get_size(root), 7);
        }
    }

    #[test]
    fn test_contains_deep_tree() {
        // Tests contains on a deep tree traversal
        let mut tree = TestTree::new(30);
        let mut root: usize = 0;

        unsafe {
            // Build a larger tree
            for i in [15, 8, 22, 4, 12, 18, 26, 2, 6, 10, 14, 16, 20, 24, 28] {
                tree.attach(&mut root, i);
            }

            // Search for nodes at various depths
            assert!(tree.contains(2, root));
            assert!(tree.contains(28, root));
            assert!(tree.contains(15, root));
            assert!(!tree.contains(1, root));
            assert!(!tree.contains(30, root));
            assert!(!tree.contains(13, root));
        }
    }
}

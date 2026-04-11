mod common;

use common::TestTree;
use platform_trees::{IterativeSizeBalancedTree, RecursiveSizeBalancedTree};

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
            tree.attach(&raw mut root, 5);
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
            tree.attach(&raw mut root, 5);
            tree.attach(&raw mut root, 3);
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
            tree.attach(&raw mut root, 5);
            tree.attach(&raw mut root, 7);
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
            tree.attach(&raw mut root, 10);
            tree.attach(&raw mut root, 5);
            tree.attach(&raw mut root, 15);
            tree.attach(&raw mut root, 3);
            tree.attach(&raw mut root, 7);
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
            tree.attach(&raw mut root, 5);
            tree.detach(&raw mut root, 5);
        }

        assert_eq!(root, 0);
    }

    #[test]
    fn test_detach_leaf_node() {
        let mut tree = TestTree::new(20);
        let mut root: usize = 0;

        unsafe {
            tree.attach(&raw mut root, 10);
            tree.attach(&raw mut root, 5);
            tree.attach(&raw mut root, 15);

            tree.detach(&raw mut root, 5);

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
            tree.attach(&raw mut root, 10);
            tree.attach(&raw mut root, 5);
            tree.attach(&raw mut root, 3);

            tree.detach(&raw mut root, 5);

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
            tree.attach(&raw mut root, 10);
            tree.attach(&raw mut root, 5);
            tree.attach(&raw mut root, 15);
            tree.attach(&raw mut root, 3);
            tree.attach(&raw mut root, 7);

            tree.detach(&raw mut root, 5);

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
            tree.attach(&raw mut root, 10);
            tree.attach(&raw mut root, 5);
            tree.attach(&raw mut root, 15);

            tree.detach(&raw mut root, 10);

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
                tree.attach(&raw mut root, i);
            }

            // Verify all nodes present
            for i in [10, 5, 15, 3, 7, 12, 17] {
                assert!(tree.contains(i, root), "Node {i} should be present");
            }

            // Remove some nodes
            tree.detach(&raw mut root, 3);
            tree.detach(&raw mut root, 17);
            tree.detach(&raw mut root, 10);

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
                tree.attach(&raw mut root, i);
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
                tree.attach(&raw mut root, i);
            }

            for i in 1..=5 {
                tree.detach(&raw mut root, i);
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
            tree.attach(&raw mut root, 10);
            tree.attach(&raw mut root, 5);
            tree.attach(&raw mut root, 8); // Goes right of 5, should trigger LR rotation

            assert!(tree.contains(10, root));
            assert!(tree.contains(5, root));
            assert!(tree.contains(8, root));
            assert_eq!(tree.get_size(root), 3);
        }
    }

    #[test]
    fn test_attach_descend_left_without_rotation() {
        let mut tree = TestTree::new(40);
        let mut root: usize = 0;

        unsafe {
            tree.attach(&raw mut root, 20);
            tree.attach(&raw mut root, 10);
            tree.attach(&raw mut root, 30);
            tree.attach(&raw mut root, 25);
            tree.attach(&raw mut root, 35);
            tree.attach(&raw mut root, 5);
            tree.attach(&raw mut root, 2);

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
            tree.attach(&raw mut root, 10);
            tree.attach(&raw mut root, 5);
            tree.attach(&raw mut root, 7);

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
            tree.attach(&raw mut root, 10);
            tree.attach(&raw mut root, 20);
            tree.attach(&raw mut root, 15); // Between 10 and 20, triggers RL rotation

            assert!(tree.contains(10, root));
            assert!(tree.contains(15, root));
            assert!(tree.contains(20, root));
            assert_eq!(tree.get_size(root), 3);
        }
    }

    #[test]
    fn test_attach_descend_right_without_rotation() {
        let mut tree = TestTree::new(40);
        let mut root: usize = 0;

        unsafe {
            tree.attach(&raw mut root, 20);
            tree.attach(&raw mut root, 30);
            tree.attach(&raw mut root, 10);
            tree.attach(&raw mut root, 5);
            tree.attach(&raw mut root, 15);
            tree.attach(&raw mut root, 35);
            tree.attach(&raw mut root, 40);

            assert!(tree.contains(40, root));
            assert_eq!(tree.get_size(root), 7);
        }
    }

    #[test]
    fn test_detach_with_double_rotation_left() {
        let mut tree = TestTree::new(40);
        let mut root: usize = 0;

        unsafe {
            for i in [20, 10, 30, 5, 15, 25, 35, 3, 8, 13, 17] {
                tree.attach(&raw mut root, i);
            }

            tree.detach(&raw mut root, 3);
            tree.detach(&raw mut root, 5);
            tree.detach(&raw mut root, 8);

            assert!(!tree.contains(3, root));
            assert!(!tree.contains(5, root));
            assert!(!tree.contains(8, root));
        }
    }

    #[test]
    fn test_detach_with_double_rotation_right() {
        let mut tree = TestTree::new(50);
        let mut root: usize = 0;

        unsafe {
            for i in [20, 10, 30, 5, 15, 25, 35, 32, 38, 28, 27] {
                tree.attach(&raw mut root, i);
            }

            tree.detach(&raw mut root, 35);
            tree.detach(&raw mut root, 38);
            tree.detach(&raw mut root, 32);

            assert!(!tree.contains(35, root));
            assert!(!tree.contains(38, root));
            assert!(!tree.contains(32, root));
        }
    }

    #[test]
    fn test_detach_with_left_only_child() {
        let mut tree = TestTree::new(30);
        let mut root: usize = 0;

        unsafe {
            tree.attach(&raw mut root, 10);
            tree.attach(&raw mut root, 5);
            tree.attach(&raw mut root, 15);
            tree.attach(&raw mut root, 3);
            tree.attach(&raw mut root, 7);
            tree.attach(&raw mut root, 2);
            tree.attach(&raw mut root, 4);

            tree.detach(&raw mut root, 5);

            assert!(!tree.contains(5, root));
            assert!(tree.contains(3, root));
            assert!(tree.contains(7, root));
        }
    }

    #[test]
    fn test_detach_node_with_left_child_only() {
        let mut tree = TestTree::new(20);
        let mut root: usize = 0;

        unsafe {
            tree.attach(&raw mut root, 10);
            tree.attach(&raw mut root, 5);
            tree.attach(&raw mut root, 15);
            tree.attach(&raw mut root, 3);

            tree.detach(&raw mut root, 5);

            assert!(!tree.contains(5, root));
            assert!(tree.contains(3, root));
        }
    }

    #[test]
    fn test_large_sequential_insert() {
        let mut tree = TestTree::new(50);
        let mut root: usize = 0;

        unsafe {
            for i in 1..=20 {
                tree.attach(&raw mut root, i);
            }

            for i in 1..=20 {
                assert!(tree.contains(i, root), "Node {i} missing");
            }
            assert_eq!(tree.get_size(root), 20);
        }
    }

    #[test]
    fn test_large_reverse_insert() {
        let mut tree = TestTree::new(50);
        let mut root: usize = 0;

        unsafe {
            for i in (1..=20).rev() {
                tree.attach(&raw mut root, i);
            }

            for i in 1..=20 {
                assert!(tree.contains(i, root), "Node {i} missing");
            }
            assert_eq!(tree.get_size(root), 20);
        }
    }

    #[test]
    fn test_alternating_insert() {
        let mut tree = TestTree::new(50);
        let mut root: usize = 0;

        unsafe {
            let mut low = 1;
            let mut high = 20;
            for i in 0..20 {
                if i % 2 == 0 {
                    tree.attach(&raw mut root, high);
                    high -= 1;
                } else {
                    tree.attach(&raw mut root, low);
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
        let mut tree = TestTree::new(100);
        let mut root: usize = 0;

        unsafe {
            for i in 1..=50 {
                tree.attach(&raw mut root, i);
            }

            for i in (1..=50).step_by(2) {
                tree.detach(&raw mut root, i);
            }

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
        let mut tree = TestTree::new(20);
        let mut root: usize = 0;

        unsafe {
            tree.attach(&raw mut root, 10);
            tree.attach(&raw mut root, 5);
            tree.attach(&raw mut root, 15);
            tree.attach(&raw mut root, 17);

            tree.detach(&raw mut root, 15);

            assert!(!tree.contains(15, root));
            assert!(tree.contains(17, root));
        }
    }

    #[test]
    fn test_attach_right_special_case_empty() {
        let mut tree = TestTree::new(20);
        let mut root: usize = 0;

        unsafe {
            tree.attach(&raw mut root, 10);
            tree.attach(&raw mut root, 20);
            tree.attach(&raw mut root, 15);

            assert!(tree.contains(10, root));
            assert!(tree.contains(15, root));
            assert!(tree.contains(20, root));
        }
    }

    #[test]
    fn test_detach_replacement_from_right_subtree() {
        let mut tree = TestTree::new(30);
        let mut root: usize = 0;

        unsafe {
            tree.attach(&raw mut root, 10);
            tree.attach(&raw mut root, 5);
            tree.attach(&raw mut root, 15);
            tree.attach(&raw mut root, 12);
            tree.attach(&raw mut root, 17);
            tree.attach(&raw mut root, 16);
            tree.attach(&raw mut root, 18);

            tree.detach(&raw mut root, 15);

            assert!(!tree.contains(15, root));
            assert!(tree.contains(12, root));
            assert!(tree.contains(17, root));
            assert!(tree.contains(16, root));
            assert!(tree.contains(18, root));
        }
    }

    #[test]
    fn test_get_rightest_single_node() {
        let mut tree = TestTree::new(10);
        unsafe {
            tree.set_size(1, 1);
            assert_eq!(tree.get_rightest(1), 1);
        }
    }

    #[test]
    fn test_get_leftest_single_node() {
        let mut tree = TestTree::new(10);
        unsafe {
            tree.set_size(1, 1);
            assert_eq!(tree.get_leftest(1), 1);
        }
    }

    #[test]
    fn test_get_left_size_no_left_child() {
        let mut tree = TestTree::new(10);
        unsafe {
            tree.set_size(1, 1);
            assert_eq!(tree.get_left_size(1), 0);
        }
    }

    #[test]
    fn test_get_right_size_no_right_child() {
        let mut tree = TestTree::new(10);
        unsafe {
            tree.set_size(1, 1);
            assert_eq!(tree.get_right_size(1), 0);
        }
    }

    #[test]
    fn test_fix_size_leaf_node() {
        let mut tree = TestTree::new(10);
        unsafe {
            tree.fix_size(1);
            assert_eq!(tree.get_size(1), 1); // 0 + 0 + 1
        }
    }

    #[test]
    fn test_detach_reinsert_cycle() {
        let mut tree = TestTree::new(30);
        let mut root: usize = 0;

        unsafe {
            let nodes = [15, 10, 20, 5, 12, 17, 25];
            for &i in &nodes {
                tree.attach(&raw mut root, i);
            }

            for &i in &nodes {
                tree.detach(&raw mut root, i);
            }
            assert_eq!(root, 0);

            for &i in &nodes {
                tree.attach(&raw mut root, i);
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
        let mut tree = TestTree::new(30);
        let mut root: usize = 0;

        unsafe {
            for i in [15, 8, 22, 4, 12, 18, 26, 2, 6, 10, 14, 16, 20, 24, 28] {
                tree.attach(&raw mut root, i);
            }

            assert!(tree.contains(2, root));
            assert!(tree.contains(28, root));
            assert!(tree.contains(15, root));
            assert!(!tree.contains(1, root));
            assert!(!tree.contains(30, root));
            assert!(!tree.contains(13, root));
        }
    }
}

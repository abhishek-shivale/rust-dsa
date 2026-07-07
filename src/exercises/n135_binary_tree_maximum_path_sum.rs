//! https://leetcode.com/problems/binary-tree-maximum-path-sum/
//!
//! A path is any sequence of nodes connected by edges, need not pass
//! through the root, each node used at most once. Return the maximum path
//! sum.

use crate::common::TreeLink;

pub fn max_path_sum(root: TreeLink) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::TreeNode;

    #[test]
    fn simple_tree() {
        let root = TreeNode::from_level_order(&[Some(1), Some(2), Some(3)]);
        assert_eq!(max_path_sum(root), 6);
    }

    #[test]
    fn negative_values() {
        let root = TreeNode::from_level_order(&[Some(-10), Some(9), Some(20), None, None, Some(15), Some(7)]);
        assert_eq!(max_path_sum(root), 42);
    }
}

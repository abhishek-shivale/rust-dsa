//! https://leetcode.com/problems/diameter-of-binary-tree/
//!
//! Return the length (in edges) of the longest path between any two nodes,
//! which may or may not pass through the root.

use crate::common::TreeLink;

pub fn diameter_of_binary_tree(root: TreeLink) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::TreeNode;

    #[test]
    fn example_tree() {
        let root = TreeNode::from_level_order(&[Some(1), Some(2), Some(3), Some(4), Some(5)]);
        assert_eq!(diameter_of_binary_tree(root), 3);
    }

    #[test]
    fn single_node() {
        let root = TreeNode::from_level_order(&[Some(1)]);
        assert_eq!(diameter_of_binary_tree(root), 0);
    }
}

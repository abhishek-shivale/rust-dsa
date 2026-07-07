//! https://leetcode.com/problems/invert-binary-tree/
//!
//! Invert a binary tree (swap left/right at every node) and return the root.

use crate::common::TreeLink;

pub fn invert_tree(root: TreeLink) -> TreeLink {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::TreeNode;

    #[test]
    fn example_tree() {
        let root = TreeNode::from_level_order(&[Some(4), Some(2), Some(7), Some(1), Some(3), Some(6), Some(9)]);
        let expected = vec![Some(4), Some(7), Some(2), Some(9), Some(6), Some(3), Some(1)];
        assert_eq!(TreeNode::to_level_order(invert_tree(root)), expected);
    }

    #[test]
    fn empty_tree() {
        assert_eq!(TreeNode::to_level_order(invert_tree(None)), Vec::new());
    }
}

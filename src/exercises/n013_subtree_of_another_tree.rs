//! https://leetcode.com/problems/subtree-of-another-tree/
//!
//! Return whether `sub_root` appears anywhere in `root` as an exact subtree.

use crate::common::TreeLink;

pub fn is_subtree(root: TreeLink, sub_root: TreeLink) -> bool {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::TreeNode;

    #[test]
    fn is_a_subtree() {
        let root = TreeNode::from_level_order(&[Some(3), Some(4), Some(5), Some(1), Some(2)]);
        let sub = TreeNode::from_level_order(&[Some(4), Some(1), Some(2)]);
        assert!(is_subtree(root, sub));
    }

    #[test]
    fn not_a_subtree() {
        let root = TreeNode::from_level_order(&[Some(3), Some(4), Some(5), Some(1), Some(2), None, None, None, None, None, Some(0)]);
        let sub = TreeNode::from_level_order(&[Some(4), Some(1), Some(2)]);
        assert!(!is_subtree(root, sub));
    }
}

//! https://leetcode.com/problems/validate-binary-search-tree/
//!
//! Return whether the tree is a valid binary search tree.

use crate::common::TreeLink;

pub fn is_valid_bst(root: TreeLink) -> bool {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::TreeNode;

    #[test]
    fn valid() {
        let root = TreeNode::from_level_order(&[Some(2), Some(1), Some(3)]);
        assert!(is_valid_bst(root));
    }

    #[test]
    fn invalid() {
        let root = TreeNode::from_level_order(&[Some(5), Some(1), Some(4), None, None, Some(3), Some(6)]);
        assert!(!is_valid_bst(root));
    }
}

//! https://leetcode.com/problems/symmetric-tree/
//!
//! Return whether the tree is a mirror image of itself around its center.

use crate::common::TreeLink;

pub fn is_symmetric(root: TreeLink) -> bool {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::TreeNode;

    #[test]
    fn symmetric() {
        let root = TreeNode::from_level_order(&[Some(1), Some(2), Some(2), Some(3), Some(4), Some(4), Some(3)]);
        assert!(is_symmetric(root));
    }

    #[test]
    fn not_symmetric() {
        let root = TreeNode::from_level_order(&[Some(1), Some(2), Some(2), None, Some(3), None, Some(3)]);
        assert!(!is_symmetric(root));
    }
}

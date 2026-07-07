//! https://leetcode.com/problems/balanced-binary-tree/
//!
//! Return whether every node's two subtrees differ in height by at most 1.

use crate::common::TreeLink;

pub fn is_balanced(root: TreeLink) -> bool {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::TreeNode;

    #[test]
    fn balanced() {
        let root = TreeNode::from_level_order(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);
        assert!(is_balanced(root));
    }

    #[test]
    fn unbalanced() {
        let root = TreeNode::from_level_order(&[
            Some(1), Some(2), Some(2), Some(3), Some(3), None, None, Some(4), Some(4),
        ]);
        assert!(!is_balanced(root));
    }
}

//! https://leetcode.com/problems/same-tree/
//!
//! Return whether two binary trees are structurally identical with the
//! same node values.

use crate::common::TreeLink;

pub fn is_same_tree(p: TreeLink, q: TreeLink) -> bool {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::TreeNode;

    #[test]
    fn identical() {
        let p = TreeNode::from_level_order(&[Some(1), Some(2), Some(3)]);
        let q = TreeNode::from_level_order(&[Some(1), Some(2), Some(3)]);
        assert!(is_same_tree(p, q));
    }

    #[test]
    fn different_values() {
        let p = TreeNode::from_level_order(&[Some(1), Some(2)]);
        let q = TreeNode::from_level_order(&[Some(1), None, Some(2)]);
        assert!(!is_same_tree(p, q));
    }
}

//! https://leetcode.com/problems/flatten-binary-tree-to-linked-list/
//!
//! Flatten the tree into a "linked list" following preorder traversal:
//! every node's `left` is `None` and `right` points to the next node.
//! Return the flattened preorder values for easy testing.

use crate::common::TreeLink;

pub fn flatten(root: TreeLink) {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::TreeNode;

    fn preorder_right_chain(mut node: TreeLink) -> Vec<i32> {
        let mut out = Vec::new();
        while let Some(n) = node {
            let n = n.borrow();
            assert!(n.left.is_none(), "left should be None after flattening");
            out.push(n.val);
            node = n.right.clone();
        }
        out
    }

    #[test]
    fn example_1() {
        let root = TreeNode::from_level_order(&[
            Some(1), Some(2), Some(5), Some(3), Some(4), None, Some(6),
        ]);
        flatten(root.clone());
        assert_eq!(preorder_right_chain(root), vec![1, 2, 3, 4, 5, 6]);
    }
}

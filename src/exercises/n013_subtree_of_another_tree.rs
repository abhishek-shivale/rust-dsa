//! https://leetcode.com/problems/subtree-of-another-tree/
//!
//! Return whether `sub_root` appears anywhere in `root` as an exact subtree.

use crate::common::TreeLink;

pub fn is_subtree(root: TreeLink, sub_root: TreeLink) -> bool {
    let mut stack = Vec::new();

    if let Some(node) = root.clone() {
        stack.push(node);
    };

    while let Some(r_node) = stack.pop() {
        if let Some(s_node) = sub_root.clone() {
            let root_ref = r_node.borrow();
            let sub_root_ref = s_node.borrow();

            if root_ref.val == sub_root_ref.val
                && root_ref.left == sub_root_ref.left
                && root_ref.right == sub_root_ref.right
            {
                return true;
            } else {
                if let Some(left_child) = root_ref.left.clone() {
                    stack.push(left_child);
                }

                if let Some(right_child) = root_ref.right.clone() {
                    stack.push(right_child);
                }
            }
        }
    };

    return false;
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
        let root = TreeNode::from_level_order(&[
            Some(3),
            Some(4),
            Some(5),
            Some(1),
            Some(2),
            None,
            None,
            None,
            None,
            None,
            Some(0),
        ]);
        let sub = TreeNode::from_level_order(&[Some(4), Some(1), Some(2)]);
        assert!(!is_subtree(root, sub));
    }
}

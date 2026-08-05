//! https://leetcode.com/problems/balanced-binary-tree/
//!
//! Return whether every node's two subtrees differ in height by at most 1.

use crate::common::TreeLink;

fn height(node: &TreeLink) -> i32 {
    let Some(node) = node else {
        return 0;
    };

    let (left, right) = {
        let node = node.borrow();
        (node.left.clone(), node.right.clone())
    };

    let left_height = height(&left);
    if left_height == -1 {
        return -1;
    }

    let right_height = height(&right);
    if right_height == -1 || (left_height - right_height).abs() > 1 {
        return -1;
    }

    1 + left_height.max(right_height)
}

pub fn is_balanced(root: TreeLink) -> bool {
    height(&root) != -1
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::TreeNode;

    #[test]
    fn balanced() {
        let root = TreeNode::from_level_order(&[
            Some(3),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ]);
        assert!(is_balanced(root));
    }

    #[test]
    fn unbalanced() {
        let root = TreeNode::from_level_order(&[
            Some(1),
            Some(2),
            Some(2),
            Some(3),
            Some(3),
            None,
            None,
            Some(4),
            Some(4),
        ]);
        assert!(!is_balanced(root));
    }
}

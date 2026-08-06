//! https://leetcode.com/problems/symmetric-tree/
//!
//! Return whether the tree is a mirror image of itself around its center.

use crate::common::TreeLink;

fn symmetric(node: &TreeLink) -> bool {
    fn mirrors(left: &TreeLink, right: &TreeLink) -> bool {
        match (left, right) {
            (None, None) => true,
            (Some(_), None) | (None, Some(_)) => false,
            (Some(left), Some(right)) => {
                let left = left.borrow();
                let right = right.borrow();

                left.val == right.val
                    && mirrors(&left.left, &right.right)
                    && mirrors(&left.right, &right.left)
            }
        }
    }

    let Some(node) = node else {
        return true;
    };

    let node = node.borrow();
    mirrors(&node.left, &node.right)
}

pub fn is_symmetric(root: TreeLink) -> bool {
    symmetric(&root)
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
        let root =
            TreeNode::from_level_order(&[Some(1), Some(2), Some(2), None, Some(3), None, Some(3)]);
        assert!(!is_symmetric(root));
    }
}

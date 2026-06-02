//! https://leetcode.com/problems/same-tree/
//!
//! Return whether two binary trees are structurally identical with the
//! same node values.

use crate::common::TreeLink;

pub fn is_same_tree(p: TreeLink, q: TreeLink) -> bool {
    let mut stack = vec![(p, q)];

    while let Some((p_node, q_node)) = stack.pop() {
        match (p_node, q_node) {
            (None, None) => {}
            (Some(_), None) | (None, Some(_)) => return false,
            (Some(p_node), Some(q_node)) => {
                let p_node = p_node.borrow();
                let q_node = q_node.borrow();

                if p_node.val != q_node.val {
                    return false;
                }

                stack.push((p_node.left.clone(), q_node.left.clone()));
                stack.push((p_node.right.clone(), q_node.right.clone()));
            }
        }
    }

    true
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

    #[test]
    fn empty_trees_are_identical() {
        assert!(is_same_tree(None, None));
    }

    #[test]
    fn preserves_input_trees() {
        let p = TreeNode::from_level_order(&[Some(1), Some(2), Some(3)]);
        let q = TreeNode::from_level_order(&[Some(1), Some(2), Some(3)]);

        assert!(is_same_tree(p.clone(), q.clone()));
        assert_eq!(TreeNode::to_level_order(p), vec![Some(1), Some(2), Some(3)]);
        assert_eq!(TreeNode::to_level_order(q), vec![Some(1), Some(2), Some(3)]);
    }
}

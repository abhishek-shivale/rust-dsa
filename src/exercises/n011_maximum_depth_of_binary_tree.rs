//! https://leetcode.com/problems/maximum-depth-of-binary-tree/
//!
//! Return the number of nodes along the longest path from root to a leaf.

use crate::common::TreeLink;

pub fn max_depth(root: TreeLink) -> i32 {
    let mut stack = Vec::new();

    if let Some(node) = root.clone() {
        stack.push((node, 1));
    };
    let mut return_depth = stack.len();
    while let Some((node, depth)) = stack.pop() {
        let node_ref = node.borrow_mut();
        let d = depth + 1;
        if let Some(left_child) = node_ref.left.clone() {
            if return_depth < d {
                return_depth += 1
            }
            stack.push((left_child, d));
        }

        if let Some(right_child) = node_ref.right.clone() {
            if return_depth < d {
                return_depth += 1
            }
            stack.push((right_child, d));
        }
    }
    return_depth as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::TreeNode;

    #[test]
    fn example_tree() {
        let root = TreeNode::from_level_order(&[
            Some(3),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ]);
        assert_eq!(max_depth(root), 3);
    }

    #[test]
    fn empty_tree() {
        let root = TreeNode::from_level_order(&[Some(1), None, Some(1)]);
        assert_eq!(max_depth(root), 2);
    }
}

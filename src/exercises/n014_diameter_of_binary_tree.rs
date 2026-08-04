//! https://leetcode.com/problems/diameter-of-binary-tree/
//!
//! Return the length (in edges) of the longest path between any two nodes,
//! which may or may not pass through the root.

use crate::common::TreeLink;

use std::{collections::HashMap, rc::Rc};

pub fn diameter_of_binary_tree(root: TreeLink) -> i32 {
    let Some(root) = root else {
        return 0;
    };

    let mut stack = vec![(root, false)];
    let mut heights = HashMap::new();
    let mut diameter = 0;

    while let Some((node, visited)) = stack.pop() {
        if !visited {
            stack.push((Rc::clone(&node), true));

            let node_ref = node.borrow();

            if let Some(right) = node_ref.right.clone() {
                stack.push((right, false));
            }

            if let Some(left) = node_ref.left.clone() {
                stack.push((left, false));
            }
        } else {
            let node_ref = node.borrow();

            let left_height = node_ref
                .left
                .as_ref()
                .and_then(|child| heights.get(&Rc::as_ptr(child)))
                .copied()
                .unwrap_or(0);

            let right_height = node_ref
                .right
                .as_ref()
                .and_then(|child| heights.get(&Rc::as_ptr(child)))
                .copied()
                .unwrap_or(0);

            diameter = diameter.max(left_height + right_height);

            let height = 1 + left_height.max(right_height);
            heights.insert(Rc::as_ptr(&node), height);
        }
    }

    diameter
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::TreeNode;

    #[test]
    fn example_tree() {
        let root = TreeNode::from_level_order(&[Some(1), Some(2), Some(3), Some(4), Some(5)]);
        assert_eq!(diameter_of_binary_tree(root), 3);
    }

    #[test]
    fn single_node() {
        let root = TreeNode::from_level_order(&[Some(1)]);
        assert_eq!(diameter_of_binary_tree(root), 0);
    }
}

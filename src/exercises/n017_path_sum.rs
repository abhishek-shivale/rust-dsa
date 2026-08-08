//! https://leetcode.com/problems/path-sum/
//!
//! Return whether some root-to-leaf path sums exactly to `target_sum`.

use crate::common::TreeLink;

pub fn has_path_sum(root: TreeLink, target_sum: i32) -> bool {
    let Some(root) = root else {
        return false;
    };

    let mut stack = vec![(root, target_sum)];

    while let Some((node, remaining)) = stack.pop() {
        let node_ref = node.borrow();
        let remaining = remaining - node_ref.val;

        let is_leaf = node_ref.left.is_none() && node_ref.right.is_none();

        if is_leaf && remaining == 0 {
            return true;
        }

        if let Some(left) = node_ref.left.clone() {
            stack.push((left, remaining));
        }

        if let Some(right) = node_ref.right.clone() {
            stack.push((right, remaining));
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::TreeNode;

    #[test]
    fn path_exists() {
        let root = TreeNode::from_level_order(&[
            Some(5), Some(4), Some(8), Some(11), None, Some(13), Some(4), Some(7), Some(2), None, None, None, Some(1),
        ]);
        assert!(has_path_sum(root, 22));
    }

    #[test]
    fn empty_tree() {
        assert!(!has_path_sum(None, 0));
    }
}

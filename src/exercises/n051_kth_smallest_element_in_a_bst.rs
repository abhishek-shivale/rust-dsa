//! https://leetcode.com/problems/kth-smallest-element-in-a-bst/
//!
//! Return the `k`th smallest value in a binary search tree (1-indexed).

use crate::common::TreeLink;

pub fn kth_smallest(root: TreeLink, k: i32) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::TreeNode;

    #[test]
    fn example_1() {
        let root = TreeNode::from_level_order(&[Some(3), Some(1), Some(4), None, Some(2)]);
        assert_eq!(kth_smallest(root, 1), 1);
    }

    #[test]
    fn example_2() {
        let root = TreeNode::from_level_order(&[Some(5), Some(3), Some(6), Some(2), Some(4), None, None, Some(1)]);
        assert_eq!(kth_smallest(root, 3), 3);
    }
}

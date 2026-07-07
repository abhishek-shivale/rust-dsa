//! https://leetcode.com/problems/binary-tree-level-order-traversal/
//!
//! Return the values of a binary tree's nodes, level by level, left to
//! right.

use crate::common::TreeLink;

pub fn level_order(root: TreeLink) -> Vec<Vec<i32>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::TreeNode;

    #[test]
    fn example_1() {
        let root = TreeNode::from_level_order(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);
        assert_eq!(level_order(root), vec![vec![3], vec![9, 20], vec![15, 7]]);
    }

    #[test]
    fn empty_tree() {
        assert_eq!(level_order(None), Vec::<Vec<i32>>::new());
    }
}

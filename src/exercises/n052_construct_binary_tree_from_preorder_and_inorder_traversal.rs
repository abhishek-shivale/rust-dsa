//! https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/
//!
//! Rebuild and return the binary tree described by its `preorder` and
//! `inorder` traversals (both contain unique values).

use crate::common::TreeLink;

pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> TreeLink {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::TreeNode;

    #[test]
    fn example_1() {
        let root = build_tree(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7]);
        let expected = TreeNode::from_level_order(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);
        assert_eq!(TreeNode::to_level_order(root), TreeNode::to_level_order(expected));
    }
}

//! https://leetcode.com/problems/maximum-depth-of-binary-tree/
//!
//! Return the number of nodes along the longest path from root to a leaf.

use crate::common::TreeLink;

pub fn max_depth(root: TreeLink) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::TreeNode;

    #[test]
    fn example_tree() {
        let root = TreeNode::from_level_order(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);
        assert_eq!(max_depth(root), 3);
    }

    #[test]
    fn empty_tree() {
        assert_eq!(max_depth(None), 0);
    }
}

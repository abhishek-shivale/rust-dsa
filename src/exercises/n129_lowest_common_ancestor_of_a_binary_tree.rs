//! https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree/
//!
//! Return the lowest common ancestor of nodes with values `p` and `q` in
//! the tree (both guaranteed present).

use crate::common::TreeLink;

pub fn lowest_common_ancestor(root: TreeLink, p: i32, q: i32) -> Option<i32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::TreeNode;

    #[test]
    fn example_1() {
        let root = TreeNode::from_level_order(&[
            Some(3), Some(5), Some(1), Some(6), Some(2), Some(0), Some(8), None, None, Some(7), Some(4),
        ]);
        assert_eq!(lowest_common_ancestor(root, 5, 1), Some(3));
    }
}

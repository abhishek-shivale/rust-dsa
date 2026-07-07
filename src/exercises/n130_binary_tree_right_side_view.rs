//! https://leetcode.com/problems/binary-tree-right-side-view/
//!
//! Return the values visible when looking at the tree from the right
//! side, top to bottom (the rightmost node at each level).

use crate::common::TreeLink;

pub fn right_side_view(root: TreeLink) -> Vec<i32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::TreeNode;

    #[test]
    fn example_1() {
        let root = TreeNode::from_level_order(&[Some(1), Some(2), Some(3), None, Some(5), None, Some(4)]);
        assert_eq!(right_side_view(root), vec![1, 3, 4]);
    }
}

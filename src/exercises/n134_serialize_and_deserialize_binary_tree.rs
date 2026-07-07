//! https://leetcode.com/problems/serialize-and-deserialize-binary-tree/
//!
//! Design an algorithm to serialize a binary tree to a string and
//! deserialize it back to the same tree structure.

use crate::common::TreeLink;

pub fn serialize(root: TreeLink) -> String {
    todo!()
}

pub fn deserialize(data: String) -> TreeLink {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::TreeNode;

    #[test]
    fn round_trips() {
        let root = TreeNode::from_level_order(&[Some(1), Some(2), Some(3), None, None, Some(4), Some(5)]);
        let expected = TreeNode::to_level_order(root.clone());
        let restored = deserialize(serialize(root));
        assert_eq!(TreeNode::to_level_order(restored), expected);
    }

    #[test]
    fn empty_tree() {
        assert_eq!(TreeNode::to_level_order(deserialize(serialize(None))), Vec::new());
    }
}

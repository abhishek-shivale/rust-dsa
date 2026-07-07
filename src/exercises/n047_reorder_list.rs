//! https://leetcode.com/problems/reorder-list/
//!
//! Reorder `L0 -> L1 -> ... -> Ln` in place into
//! `L0 -> Ln -> L1 -> Ln-1 -> L2 -> ...`.

use crate::common::ListNode;

pub fn reorder_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn even_length() {
        let head = ListNode::from_vec(&[1, 2, 3, 4]);
        assert_eq!(ListNode::to_vec(reorder_list(head)), vec![1, 4, 2, 3]);
    }

    #[test]
    fn odd_length() {
        let head = ListNode::from_vec(&[1, 2, 3, 4, 5]);
        assert_eq!(ListNode::to_vec(reorder_list(head)), vec![1, 5, 2, 4, 3]);
    }
}

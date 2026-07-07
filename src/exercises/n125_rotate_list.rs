//! https://leetcode.com/problems/rotate-list/
//!
//! Rotate the linked list right by `k` places and return the new head.

use crate::common::ListNode;

pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let head = ListNode::from_vec(&[1, 2, 3, 4, 5]);
        assert_eq!(ListNode::to_vec(rotate_right(head, 2)), vec![4, 5, 1, 2, 3]);
    }

    #[test]
    fn k_equals_len() {
        let head = ListNode::from_vec(&[0, 1, 2]);
        assert_eq!(ListNode::to_vec(rotate_right(head, 4)), vec![2, 0, 1]);
    }
}

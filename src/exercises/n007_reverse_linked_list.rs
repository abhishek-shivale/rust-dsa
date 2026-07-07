//! https://leetcode.com/problems/reverse-linked-list/
//!
//! Reverse a singly linked list and return the new head.

use crate::common::ListNode;

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn several_nodes() {
        let head = ListNode::from_vec(&[1, 2, 3, 4, 5]);
        assert_eq!(ListNode::to_vec(reverse_list(head)), vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn empty_list() {
        assert_eq!(ListNode::to_vec(reverse_list(None)), Vec::<i32>::new());
    }
}

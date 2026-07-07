//! https://leetcode.com/problems/remove-nth-node-from-end-of-list/
//!
//! Remove the `n`th node from the end of the list and return the head.

use crate::common::ListNode;

pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let head = ListNode::from_vec(&[1, 2, 3, 4, 5]);
        assert_eq!(ListNode::to_vec(remove_nth_from_end(head, 2)), vec![1, 2, 3, 5]);
    }

    #[test]
    fn single_node() {
        let head = ListNode::from_vec(&[1]);
        assert_eq!(ListNode::to_vec(remove_nth_from_end(head, 1)), Vec::<i32>::new());
    }
}

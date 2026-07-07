//! https://leetcode.com/problems/merge-two-sorted-lists/
//!
//! Merge two sorted linked lists into one sorted linked list.

use crate::common::ListNode;

pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interleaved() {
        let l1 = ListNode::from_vec(&[1, 2, 4]);
        let l2 = ListNode::from_vec(&[1, 3, 4]);
        assert_eq!(
            ListNode::to_vec(merge_two_lists(l1, l2)),
            vec![1, 1, 2, 3, 4, 4]
        );
    }

    #[test]
    fn both_empty() {
        assert_eq!(ListNode::to_vec(merge_two_lists(None, None)), Vec::<i32>::new());
    }
}

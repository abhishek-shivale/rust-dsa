//! https://leetcode.com/problems/merge-k-sorted-lists/
//!
//! Merge `k` sorted linked lists into one sorted linked list.

use crate::common::ListNode;

pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let lists = vec![
            ListNode::from_vec(&[1, 4, 5]),
            ListNode::from_vec(&[1, 3, 4]),
            ListNode::from_vec(&[2, 6]),
        ];
        assert_eq!(
            ListNode::to_vec(merge_k_lists(lists)),
            vec![1, 1, 2, 3, 4, 4, 5, 6]
        );
    }

    #[test]
    fn all_empty() {
        assert_eq!(ListNode::to_vec(merge_k_lists(vec![None, None])), Vec::<i32>::new());
    }
}

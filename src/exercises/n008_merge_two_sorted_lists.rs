//! https://leetcode.com/problems/merge-two-sorted-lists/
//!
//! Merge two sorted linked lists into one sorted linked list.

use crate::common::ListNode;

pub fn merge_two_lists(
    mut list1: Option<Box<ListNode>>,
    mut list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut new_vec = Vec::new();

    while let Some(val) = list1 {
        new_vec.push(val.val);
        list1 = val.next;
    }

    while let Some(val) = list2 {
        new_vec.push(val.val);
        list2 = val.next;
    }

    let mut current: Option<Box<ListNode>> = None;
    new_vec.sort_by(|a, b| b.cmp(a));
    for i in new_vec.iter() {
        let new_node = ListNode {
            next: current,
            val: i.clone(),
        };
        current = Some(Box::new(new_node));
    }
    current
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
        assert_eq!(
            ListNode::to_vec(merge_two_lists(None, None)),
            Vec::<i32>::new()
        );
    }
}

//! https://leetcode.com/problems/reverse-linked-list/
//!
//! Reverse a singly linked list and return the new head.

use crate::common::ListNode;

pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut values = Vec::new();
       while let Some(node) = head {
            values.push(node.val);
            head = node.next;
        }

    // let mut head: Option<Box<ListNode>> = None;
    // for &v in values.iter() {
    //     let mut node = Box::new(ListNode::new(v));
    //     node.next = head;
    //     head = Some(node);
    // }
    let mut head: Option<Box<ListNode>> = None;
    for &v in values.iter() {
        let new_node = Box::new(ListNode{val: v, next: head} );
        head = Some(new_node);
    };
    head
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

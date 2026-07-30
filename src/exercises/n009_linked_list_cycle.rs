//! https://leetcode.com/problems/linked-list-cycle/
//!
//! Return whether the linked list has a cycle. Build one for the test by
//! wiring a tail node's `next` back to an earlier node manually.

use crate::common::ListNode;

pub fn has_cycle(mut head: Option<Box<ListNode>>) -> bool {
    while let Some(val) = head {
       head = val.next;
    };

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_cycle() {
        let head = ListNode::from_vec(&[1, 2, 3]);
        assert!(!has_cycle(head));
    }

    #[test]
    fn empty_list() {
        assert!(!has_cycle(None));
    }
}

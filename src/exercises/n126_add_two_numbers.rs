//! https://leetcode.com/problems/add-two-numbers/
//!
//! Each list stores a non-negative integer's digits in reverse order.
//! Return their sum as a linked list in the same format.

use crate::common::ListNode;

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let l1 = ListNode::from_vec(&[2, 4, 3]);
        let l2 = ListNode::from_vec(&[5, 6, 4]);
        assert_eq!(ListNode::to_vec(add_two_numbers(l1, l2)), vec![7, 0, 8]);
    }

    #[test]
    fn with_carry_out() {
        let l1 = ListNode::from_vec(&[9, 9]);
        let l2 = ListNode::from_vec(&[1]);
        assert_eq!(ListNode::to_vec(add_two_numbers(l1, l2)), vec![0, 0, 1]);
    }
}

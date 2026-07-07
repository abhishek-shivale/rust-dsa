//! https://leetcode.com/problems/odd-even-linked-list/
//!
//! Group all odd-indexed nodes together followed by all even-indexed
//! nodes (1-indexed), preserving relative order within each group.

use crate::common::ListNode;

pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let head = ListNode::from_vec(&[1, 2, 3, 4, 5]);
        assert_eq!(ListNode::to_vec(odd_even_list(head)), vec![1, 3, 5, 2, 4]);
    }
}

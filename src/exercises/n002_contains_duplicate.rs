//! https://leetcode.com/problems/contains-duplicate/
//!
//! Return `true` if any value appears at least twice in `nums`.

use std::collections::HashSet;

pub fn has_duplicate(nums: Vec<i32>) -> bool {
    let mut seen = HashSet::with_capacity(nums.len());

    for value in nums {
        if !seen.insert(value) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_dupe() {
        assert!(has_duplicate(vec![1, 2, 3, 1]));
    }

    #[test]
    fn all_distinct() {
        assert!(!has_duplicate(vec![1, 2, 3, 4]));
    }

    #[test]
    fn empty() {
        assert!(!has_duplicate(vec![]));
    }
}

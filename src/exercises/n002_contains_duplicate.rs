//! https://leetcode.com/problems/contains-duplicate/
//!
//! Return `true` if any value appears at least twice in `nums`.

use std::collections::{HashMap, HashSet};

pub fn has_duplicate(nums: Vec<i32>) -> bool {
    let mut dup = HashSet::with_capacity(nums.len());
    for value in nums {
        let value = value;
        if dup.contains(&value.clone()) {
            return true;
        } else {
            dup.insert(value);
        }
    };

    return  false;
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
}

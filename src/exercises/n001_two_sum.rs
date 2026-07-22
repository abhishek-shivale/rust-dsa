//! https://leetcode.com/problems/two-sum/
//!
//! Given `nums` and `target`, return the indices of the two numbers that add
//! up to `target`. Exactly one answer exists; don't reuse an element.

use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut seen: HashMap<i32, usize> = HashMap::with_capacity(nums.len());

    for (index, &number) in nums.iter().enumerate() {
        if let Some(remaining) = target.checked_sub(number) {
            if let Some(&found) = seen.get(&remaining) {
                return vec![found as i32, index as i32];
            }
        };

        seen.insert(number, index);
    };
Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }

    #[test]
    fn later_pair() {
        assert_eq!(two_sum(vec![3, 2, 3], 6), vec![0, 2]);
    }

    #[test]
    fn duplicate_values() {
        assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
    }

    #[test]
    fn no_pair() {
        assert!(two_sum(vec![], 0).is_empty());
    }
}

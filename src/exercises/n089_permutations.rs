//! https://leetcode.com/problems/permutations/
//!
//! `nums` has unique values. Return all possible permutations, in any
//! order.

pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn three_elements() {
        let result: HashSet<Vec<i32>> = permute(vec![1, 2, 3]).into_iter().collect();
        assert_eq!(result.len(), 6);
        assert!(result.contains(&vec![1, 2, 3]));
        assert!(result.contains(&vec![3, 2, 1]));
    }
}

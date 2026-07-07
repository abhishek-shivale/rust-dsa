//! https://leetcode.com/problems/subsets/
//!
//! `nums` has unique values. Return the power set (all subsets), in any
//! order.

pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn example_1() {
        let mut result: Vec<Vec<i32>> = subsets(vec![1, 2, 3]);
        for s in result.iter_mut() {
            s.sort();
        }
        let result: HashSet<Vec<i32>> = result.into_iter().collect();
        let expected: HashSet<Vec<i32>> = vec![
            vec![], vec![1], vec![2], vec![3], vec![1, 2], vec![1, 3], vec![2, 3], vec![1, 2, 3],
        ]
        .into_iter()
        .collect();
        assert_eq!(result, expected);
    }
}

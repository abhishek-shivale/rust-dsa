//! https://leetcode.com/problems/rotate-array/
//!
//! Rotate `nums` right by `k` steps, in place. Return the mutated vec for
//! easy testing.

pub fn rotate(nums: Vec<i32>, k: i32) -> Vec<i32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(rotate(vec![1, 2, 3, 4, 5, 6, 7], 3), vec![5, 6, 7, 1, 2, 3, 4]);
    }

    #[test]
    fn k_larger_than_len() {
        assert_eq!(rotate(vec![-1, -100, 3, 99], 4), vec![-1, -100, 3, 99]);
    }
}

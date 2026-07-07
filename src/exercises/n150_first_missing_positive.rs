//! https://leetcode.com/problems/first-missing-positive/
//!
//! Return the smallest missing positive integer from `nums`, in O(n) time
//! and O(1) extra space.

pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(first_missing_positive(vec![1, 2, 0]), 3);
    }

    #[test]
    fn example_2() {
        assert_eq!(first_missing_positive(vec![3, 4, -1, 1]), 2);
    }
}

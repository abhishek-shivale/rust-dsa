//! https://leetcode.com/problems/longest-increasing-subsequence/
//!
//! Return the length of the longest strictly increasing subsequence of
//! `nums`.

pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
    }

    #[test]
    fn all_equal() {
        assert_eq!(length_of_lis(vec![7, 7, 7, 7]), 1);
    }
}

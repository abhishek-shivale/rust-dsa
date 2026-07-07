//! https://leetcode.com/problems/house-robber/
//!
//! `nums[i]` is money in house `i`. Adjacent houses can't both be robbed.
//! Return the max total takeable.

pub fn rob(nums: Vec<i32>) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(rob(vec![1, 2, 3, 1]), 4);
    }

    #[test]
    fn example_2() {
        assert_eq!(rob(vec![2, 7, 9, 3, 1]), 12);
    }
}

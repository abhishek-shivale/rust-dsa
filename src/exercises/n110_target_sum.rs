//! https://leetcode.com/problems/target-sum/
//!
//! Assign `+` or `-` to each element of `nums` so the expression evaluates
//! to `target`. Return the number of ways to do so.

pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(find_target_sum_ways(vec![1, 1, 1, 1, 1], 3), 5);
    }

    #[test]
    fn single_zero() {
        assert_eq!(find_target_sum_ways(vec![1], 1), 1);
    }
}

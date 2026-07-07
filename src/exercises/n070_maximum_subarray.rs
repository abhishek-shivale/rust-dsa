//! https://leetcode.com/problems/maximum-subarray/
//!
//! Return the largest sum of a contiguous non-empty subarray of `nums`
//! (Kadane's algorithm).

pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
    }

    #[test]
    fn all_negative() {
        assert_eq!(max_sub_array(vec![-1]), -1);
    }
}

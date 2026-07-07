//! https://leetcode.com/problems/maximum-product-subarray/
//!
//! Return the largest product of a contiguous non-empty subarray of
//! `nums`.

pub fn max_product(nums: Vec<i32>) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(max_product(vec![2, 3, -2, 4]), 6);
    }

    #[test]
    fn negative_flip() {
        assert_eq!(max_product(vec![-2, 0, -1]), 0);
    }
}

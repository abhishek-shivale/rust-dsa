//! https://leetcode.com/problems/product-of-array-except-self/
//!
//! Return `answer` where `answer[i]` is the product of every element of
//! `nums` except `nums[i]`. No division allowed, O(n) time.

pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(product_except_self(vec![1, 2, 3, 4]), vec![24, 12, 8, 6]);
    }

    #[test]
    fn with_zero() {
        assert_eq!(product_except_self(vec![-1, 1, 0, -3, 3]), vec![0, 0, 9, 0, 0]);
    }
}

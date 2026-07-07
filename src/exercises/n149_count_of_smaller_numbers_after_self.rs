//! https://leetcode.com/problems/count-of-smaller-numbers-after-self/
//!
//! Return `counts` where `counts[i]` is the number of elements to the
//! right of `nums[i]` that are strictly smaller than it.

pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(count_smaller(vec![5, 2, 6, 1]), vec![2, 1, 1, 0]);
    }

    #[test]
    fn already_sorted() {
        assert_eq!(count_smaller(vec![1, 2, 3]), vec![0, 0, 0]);
    }
}

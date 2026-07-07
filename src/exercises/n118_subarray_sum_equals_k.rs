//! https://leetcode.com/problems/subarray-sum-equals-k/
//!
//! Return the number of contiguous subarrays of `nums` summing to `k`.

pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(subarray_sum(vec![1, 1, 1], 2), 2);
    }

    #[test]
    fn with_negatives() {
        assert_eq!(subarray_sum(vec![1, 2, 3], 3), 2);
    }
}

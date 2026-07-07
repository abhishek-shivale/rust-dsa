//! https://leetcode.com/problems/kth-largest-element-in-an-array/
//!
//! Return the `k`th largest element in `nums` (1st largest = max), without
//! necessarily sorting the whole array.

pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2), 5);
    }

    #[test]
    fn with_duplicates() {
        assert_eq!(find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4), 4);
    }
}

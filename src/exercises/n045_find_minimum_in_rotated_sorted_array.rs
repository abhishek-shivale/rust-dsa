//! https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/
//!
//! A sorted array (no duplicates) was rotated between 1 and n times. Return
//! the minimum element in O(log n).

pub fn find_min(nums: Vec<i32>) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(find_min(vec![3, 4, 5, 1, 2]), 1);
    }

    #[test]
    fn already_sorted() {
        assert_eq!(find_min(vec![11, 13, 15, 17]), 11);
    }
}

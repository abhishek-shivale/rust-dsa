//! https://leetcode.com/problems/search-in-rotated-sorted-array/
//!
//! A sorted, distinct-valued array was rotated at an unknown pivot. Return
//! the index of `target`, or -1, in O(log n).

pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn found() {
        assert_eq!(search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
    }

    #[test]
    fn not_found() {
        assert_eq!(search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
    }
}

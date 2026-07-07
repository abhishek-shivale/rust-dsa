//! https://leetcode.com/problems/binary-search/
//!
//! `nums` is sorted ascending, distinct values. Return the index of
//! `target`, or -1 if absent. Must run in O(log n).

pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn found() {
        assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
    }

    #[test]
    fn not_found() {
        assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
    }
}

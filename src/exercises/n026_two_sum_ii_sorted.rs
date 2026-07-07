//! https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/
//!
//! `numbers` is sorted ascending. Return the 1-indexed positions of the two
//! entries that sum to `target`.

pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![1, 2]);
    }

    #[test]
    fn negative_numbers() {
        assert_eq!(two_sum(vec![-1, 0], -1), vec![1, 2]);
    }
}

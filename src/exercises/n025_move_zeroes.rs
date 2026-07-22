//! https://leetcode.com/problems/move-zeroes/
//!
//! Move all `0`s to the end of `nums` in place, keeping the relative order
//! of the non-zero elements. Return the mutated vec for easy testing.

pub fn move_zeroes(mut nums: Vec<i32>) -> Vec<i32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mixed() {
        assert_eq!(move_zeroes(vec![0, 1, 0, 3, 12]), vec![1, 3, 12, 0, 0]);
    }

    #[test]
    fn single_zero() {
        assert_eq!(move_zeroes(vec![0]), vec![0]);
    }
}

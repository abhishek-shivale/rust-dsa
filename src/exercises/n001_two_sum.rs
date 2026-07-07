//! https://leetcode.com/problems/two-sum/
//!
//! Given `nums` and `target`, return the indices of the two numbers that add
//! up to `target`. Exactly one answer exists; don't reuse an element.

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }

    #[test]
    fn later_pair() {
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    }
}

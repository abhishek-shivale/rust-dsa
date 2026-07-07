//! https://leetcode.com/problems/longest-consecutive-sequence/
//!
//! Return the length of the longest run of consecutive integers present in
//! `nums`, in O(n) time.

pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
    }

    #[test]
    fn example_2() {
        assert_eq!(
            longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]),
            9
        );
    }
}

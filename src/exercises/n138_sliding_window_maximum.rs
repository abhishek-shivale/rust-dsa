//! https://leetcode.com/problems/sliding-window-maximum/
//!
//! For each window of size `k` sliding left to right across `nums`, return
//! the maximum in that window.

pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3),
            vec![3, 3, 5, 5, 6, 7]
        );
    }

    #[test]
    fn window_equals_len() {
        assert_eq!(max_sliding_window(vec![1, -1], 1), vec![1, -1]);
    }
}

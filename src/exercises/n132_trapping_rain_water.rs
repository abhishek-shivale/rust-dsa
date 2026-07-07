//! https://leetcode.com/problems/trapping-rain-water/
//!
//! `height[i]` is a bar of that height at position `i`. Return the total
//! water trapped between the bars after rain.

pub fn trap(height: Vec<i32>) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
    }

    #[test]
    fn no_trapping() {
        assert_eq!(trap(vec![1, 2, 3]), 0);
    }
}

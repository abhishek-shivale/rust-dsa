//! https://leetcode.com/problems/container-with-most-water/
//!
//! `height[i]` is a vertical line at `x = i`. Pick two lines that, with the
//! x-axis, form a container holding the most water. Return that area.

pub fn max_area(height: Vec<i32>) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }

    #[test]
    fn two_lines() {
        assert_eq!(max_area(vec![1, 1]), 1);
    }
}

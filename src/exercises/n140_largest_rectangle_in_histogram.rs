//! https://leetcode.com/problems/largest-rectangle-in-histogram/
//!
//! `heights[i]` is a bar of width 1 at position `i`. Return the area of the
//! largest rectangle achievable within the histogram.

pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]), 10);
    }

    #[test]
    fn flat_histogram() {
        assert_eq!(largest_rectangle_area(vec![2, 4]), 4);
    }
}

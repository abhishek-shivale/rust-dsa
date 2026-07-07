//! https://leetcode.com/problems/spiral-matrix/
//!
//! Return all elements of `matrix` in spiral order (clockwise, from the
//! outside in).

pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let m = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(spiral_order(m), vec![1, 2, 3, 6, 9, 8, 7, 4, 5]);
    }
}

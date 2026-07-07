//! https://leetcode.com/problems/rotate-image/
//!
//! Rotate the `n x n` matrix 90 degrees clockwise, in place. Return it for
//! easy testing.

pub fn rotate(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_by_three() {
        let m = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(rotate(m), vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]]);
    }
}

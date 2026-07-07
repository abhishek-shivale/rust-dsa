//! https://leetcode.com/problems/spiral-matrix-ii/
//!
//! Return an `n x n` matrix filled with `1..=n*n` in spiral order.

pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            generate_matrix(3),
            vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]]
        );
    }
}

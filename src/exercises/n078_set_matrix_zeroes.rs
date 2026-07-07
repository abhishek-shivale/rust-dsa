//! https://leetcode.com/problems/set-matrix-zeroes/
//!
//! If a cell is `0`, set its entire row and column to `0`, in place.
//! Return the mutated matrix for easy testing.

pub fn set_zeroes(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let m = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        assert_eq!(
            set_zeroes(m),
            vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]]
        );
    }
}

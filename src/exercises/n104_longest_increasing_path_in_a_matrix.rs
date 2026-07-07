//! https://leetcode.com/problems/longest-increasing-path-in-a-matrix/
//!
//! Return the length of the longest strictly increasing path through
//! `matrix`, moving 4-directionally.

pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let m = vec![vec![9, 9, 4], vec![6, 6, 8], vec![2, 1, 1]];
        assert_eq!(longest_increasing_path(m), 4);
    }
}

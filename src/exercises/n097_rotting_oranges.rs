//! https://leetcode.com/problems/rotting-oranges/
//!
//! `grid` cells are 0 (empty), 1 (fresh), or 2 (rotten). Each minute, a
//! rotten orange rots its 4-directional fresh neighbors. Return the
//! minutes until no fresh orange remains, or -1 if impossible.

pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let grid = vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]];
        assert_eq!(oranges_rotting(grid), 4);
    }

    #[test]
    fn impossible() {
        let grid = vec![vec![2, 1, 1], vec![0, 1, 1], vec![1, 0, 1]];
        assert_eq!(oranges_rotting(grid), -1);
    }
}

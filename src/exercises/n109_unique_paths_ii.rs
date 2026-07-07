//! https://leetcode.com/problems/unique-paths-ii/
//!
//! Like Unique Paths, but `obstacle_grid[r][c] == 1` marks a blocked cell.
//! Return the number of distinct paths from top-left to bottom-right.

pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let grid = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
        assert_eq!(unique_paths_with_obstacles(grid), 2);
    }

    #[test]
    fn blocked_start() {
        assert_eq!(unique_paths_with_obstacles(vec![vec![1]]), 0);
    }
}

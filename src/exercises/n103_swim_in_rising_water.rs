//! https://leetcode.com/problems/swim-in-rising-water/
//!
//! `grid[r][c]` is the elevation at that cell; at time `t` you may swim
//! between adjacent cells whose elevations are both `<= t`. Return the
//! minimum time to swim from the top-left to the bottom-right.

pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(swim_in_water(vec![vec![0, 2], vec![1, 3]]), 3);
    }
}

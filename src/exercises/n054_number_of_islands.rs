//! https://leetcode.com/problems/number-of-islands/
//!
//! `grid` is a 2D map of `'1'` (land) and `'0'` (water). Return the number
//! of islands (connected land, 4-directionally).

pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn grid(rows: &[&str]) -> Vec<Vec<char>> {
        rows.iter().map(|r| r.chars().collect()).collect()
    }

    #[test]
    fn one_island() {
        let g = grid(&["11110", "11010", "11000", "00000"]);
        assert_eq!(num_islands(g), 1);
    }

    #[test]
    fn three_islands() {
        let g = grid(&["11000", "11000", "00100", "00011"]);
        assert_eq!(num_islands(g), 3);
    }
}

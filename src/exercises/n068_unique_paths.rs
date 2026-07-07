//! https://leetcode.com/problems/unique-paths/
//!
//! A robot starts top-left of an `m x n` grid, moving only right or down.
//! Return the number of distinct paths to the bottom-right.

pub fn unique_paths(m: i32, n: i32) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(unique_paths(3, 7), 28);
    }

    #[test]
    fn square() {
        assert_eq!(unique_paths(3, 2), 3);
    }
}

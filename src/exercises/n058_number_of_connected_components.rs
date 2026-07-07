//! https://leetcode.com/problems/number-of-connected-components-in-an-undirected-graph/ (Premium)
//!
//! Given `n` nodes labeled `0..n` and undirected `edges`, return the number
//! of connected components.

pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_components() {
        assert_eq!(count_components(5, vec![vec![0, 1], vec![1, 2], vec![3, 4]]), 2);
    }

    #[test]
    fn one_component() {
        assert_eq!(
            count_components(5, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4]]),
            1
        );
    }
}
